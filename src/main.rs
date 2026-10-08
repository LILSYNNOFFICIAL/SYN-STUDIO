use iced::{
    alignment, border, mouse, time, touch,
    widget::{
        button, canvas, column, container, pick_list, progress_bar,
        row, scrollable, slider, space, text, text_editor,
    },
    Color, Element, Font, Length, Point, Rectangle, Renderer, Size, Subscription, Theme,
};
use iced::widget::canvas::{Frame, Geometry, Path, Program, Stroke, Text as CanvasText};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{f32::consts::PI, time::Duration};
use uuid::Uuid;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

const BG: Color = Color::from_rgb(0.035, 0.037, 0.041);
const SURFACE: Color = Color::from_rgb(0.055, 0.058, 0.063);
const SURFACE_2: Color = Color::from_rgb(0.075, 0.078, 0.084);
const LINE: Color = Color::from_rgb(0.15, 0.16, 0.18);
const TEXT: Color = Color::from_rgb(0.94, 0.94, 0.95);
const MUTED: Color = Color::from_rgb(0.49, 0.51, 0.55);
const ACCENT: Color = Color::from_rgb(0.66, 0.62, 0.98);
const PINK: Color = Color::from_rgb(0.72, 0.67, 0.96);
const GOOD: Color = Color::from_rgb(0.43, 0.78, 0.61);

fn parse_hex(hex:&str)->Color { let clean=hex.trim_start_matches('#'); u32::from_str_radix(clean,16).ok().map(rgb).unwrap_or(TEXT) }

fn rgb(hex: u32) -> Color {
    Color::from_rgb(
        ((hex >> 16) & 0xff) as f32 / 255.0,
        ((hex >> 8) & 0xff) as f32 / 255.0,
        (hex & 0xff) as f32 / 255.0,
    )
}

fn id(prefix: &str) -> String {
    format!("{}-{}", prefix, &Uuid::new_v4().simple().to_string()[..8])
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Surface {
    Design,
    Motion,
    Architecture,
    Media,
    Code,
    Ai,
    Publish,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tool {
    Select,
    Draw,
    Text,
    Shape,
    Camera,
    Bone,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum Ease {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    Bezier,
    Spring,
}
impl Ease {
    fn apply(self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        match self {
            Self::Linear => t,
            Self::EaseIn => t * t,
            Self::EaseOut => 1.0 - (1.0 - t) * (1.0 - t),
            Self::EaseInOut => if t < 0.5 { 2.0 * t * t } else { 1.0 - (-2.0 * t + 2.0).powi(2) / 2.0 },
            Self::Bezier => t * t * (3.0 - 2.0 * t),
            Self::Spring => {
                let d = (-7.0 * t).exp();
                1.0 - d * (1.0 + 7.0 * t)
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
struct Transform3D {
    x: f32, y: f32, z: f32,
    rx: f32, ry: f32, rz: f32,
    sx: f32, sy: f32, sz: f32,
}
impl Default for Transform3D {
    fn default() -> Self { Self { x:0.0,y:0.0,z:0.0,rx:0.0,ry:0.0,rz:0.0,sx:1.0,sy:1.0,sz:1.0 } }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Keyframe {
    id: String,
    time: f32,
    value: f32,
    easing: Ease,
}
impl Keyframe {
    fn new(time: f32, value: f32) -> Self {
        Self { id: id("key"), time, value, easing: Ease::EaseInOut }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Track {
    id: String,
    target: String,
    property: String,
    keyframes: Vec<Keyframe>,
    muted: bool,
    locked: bool,
}
impl Track {
    fn value_at(&self, time: f32, fallback: f32) -> f32 {
        if self.keyframes.is_empty() { return fallback; }
        let mut keys = self.keyframes.clone();
        keys.sort_by(|a,b| a.time.total_cmp(&b.time));
        if time <= keys[0].time { return keys[0].value; }
        if time >= keys[keys.len()-1].time { return keys[keys.len()-1].value; }
        for pair in keys.windows(2) {
            let a=&pair[0]; let b=&pair[1];
            if (a.time..=b.time).contains(&time) {
                let span=(b.time-a.time).max(0.0001);
                let p=((time-a.time)/span).clamp(0.0,1.0);
                return a.value+(b.value-a.value)*b.easing.apply(p);
            }
        }
        fallback
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AnimationState {
    duration: f32,
    fps: u32,
    tracks: Vec<Track>,
    looped: bool,
    onion_skin: bool,
}
impl Default for AnimationState {
    fn default() -> Self {
        Self {
            duration: 8.0,
            fps: 60,
            tracks: vec![],
            looped: true,
            onion_skin: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Object {
    id: String,
    kind: String,
    label: String,
    x: f32, y: f32, width: f32, height: f32,
    rotation: f32,
    opacity: f32,
    locked: bool,
    hidden: bool,
    transform3d: Transform3D,
    points: Vec<[f32;2]>,
    props: Value,
}
impl Object {
    fn rect(id: &str, label: &str, x:f32,y:f32,w:f32,h:f32, kind:&str) -> Self {
        Self {
            id:id.into(), kind:kind.into(), label:label.into(), x,y,width:w,height:h,
            rotation:0.0, opacity:1.0, locked:false, hidden:false,
            transform3d:Transform3D::default(), points:vec![],
            props:json!({"color":"#8f7cff"}),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Scene {
    id: String,
    name: String,
    width: f32,
    height: f32,
    objects: Vec<Object>,
    animation: AnimationState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Document {
    syn: String,
    title: String,
    scenes: Vec<Scene>,
    active_scene: usize,
}
impl Document {
    fn sample() -> Self {
        let mut scene=Scene {
            id:"SCENE-01".into(), name:"Command Center".into(), width:1200.0,height:700.0,
            objects:vec![], animation:AnimationState::default()
        };
        scene.objects.push(Object::rect("hero","Hero",60.,60.,1080.,580.,"shape"));
        scene.objects.push(Object::rect("headline","SYN STUDIO",110.,120.,440.,90.,"text"));
        scene.objects.push(Object::rect("orb","Motion Orb",720.,170.,170.,170.,"shape"));
        let mut cube=Object::rect("cube","3D Hero Model",0.,0.,0.,0.,"model3d");
        cube.transform3d.x=820.; cube.transform3d.y=420.; cube.transform3d.z=80.;
        cube.transform3d.sx=1.35; cube.transform3d.sy=1.35; cube.transform3d.sz=1.35;
        scene.objects.push(cube);
        let mut drawing=Object::rect("stroke","Bezier Drawing",300.,420.,0.,0.,"drawing");
        drawing.points=vec![[0.,0.],[80.,-70.],[170.,70.],[260.,0.],[350.,-50.]];
        scene.objects.push(drawing);

        let mut orb_track=Track{id:id("trk"),target:"orb".into(),property:"x".into(),keyframes:vec![],muted:false,locked:false};
        orb_track.keyframes=vec![Keyframe::new(0.,720.),Keyframe::new(2.,890.),Keyframe::new(4.,720.),Keyframe::new(6.,560.),Keyframe::new(8.,720.)];
        let mut rot=Track{id:id("trk"),target:"cube".into(),property:"rz".into(),keyframes:vec![],muted:false,locked:false};
        rot.keyframes=vec![Keyframe::new(0.,0.),Keyframe::new(8.,360.)];
        let mut y=Track{id:id("trk"),target:"cube".into(),property:"y".into(),keyframes:vec![],muted:false,locked:false};
        y.keyframes=vec![Keyframe::new(0.,390.),Keyframe::new(4.,310.),Keyframe::new(8.,390.)];
        scene.animation.tracks=vec![orb_track,rot,y];

        Self{syn:"0.3".into(),title:"SYN Studio / Creative Systems".into(),scenes:vec![scene],active_scene:0}
    }
    fn scene(&self)->&Scene { &self.scenes[self.active_scene] }
}

#[derive(Debug, Clone)]
enum Message {
    Surface(Surface),
    Tool(Tool),
    SelectObject(String),
    ToggleInspector,
    InspectorSize(i32),
    PlayPause,
    Stop,
    Tick,
    SetPlayhead(f32),
    AddKeyframe,
    AddTrack(String),
    SetEase(Ease),
    SetFps(u32),
    TextSize(f32),
    TextColor(String),
    TextAlign(String),
    TextFormat(String),
    ToggleLoop(bool),
    ToggleOnion(bool),
    Save,
    New,
    Duplicate,
    Delete,
    Undo,
    Redo,
    Validate,
    Export,
    Preview,
    CodeEdit(text_editor::Action),
    ApplyCode,
    FormatCode,
    Status(String),
    Zoom(f32),
    TimelineZoom(f32),
    ToggleMenu,
    CloseOverlay,
    OpenWorkspace(String),
}

struct App {
    doc: Document,
    surface: Surface,
    tool: Tool,
    selected: Option<String>,
    inspector: bool,
    inspector_width: u16,
    playhead: f32,
    playing: bool,
    timeline_zoom: f32,
    zoom: f32,
    status: String,
    code: text_editor::Content,
    overlay: Option<String>,
    menu_open: bool,
    last_tick: Option<iced::time::Instant>,
    history: Vec<Document>,
    future: Vec<Document>,
}
impl Default for App {
    fn default() -> Self {
        let doc=load_document().unwrap_or_else(Document::sample);
        let code=text_editor::Content::with_text(&serde_json::to_string_pretty(&doc).unwrap_or_default());
        Self {
            doc, surface:Surface::Design, tool:Tool::Select, selected:None,
            inspector:false, inspector_width:330, playhead:0., playing:false,
            timeline_zoom:1.0, zoom:0.72, status:"Ready".into(), code,
            overlay:None, menu_open:false, last_tick:None, history:vec![], future:vec![],
        }
    }
}

impl App {
    fn scene_mut(&mut self)->&mut Scene { &mut self.doc.scenes[self.doc.active_scene] }
    fn selected_object_mut(&mut self)->Option<&mut Object> {
        let id=self.selected.clone()?;
        self.scene_mut().objects.iter_mut().find(|o|o.id==id)
    }
    fn snapshot(&mut self) {
        self.history.push(self.doc.clone());
        if self.history.len()>100 { self.history.remove(0); }
        self.future.clear();
    }
    fn animated_object(&self, object:&Object)->Object {
        let mut out=object.clone();
        let time=self.playhead;
        for track in &self.doc.scene().animation.tracks {
            if track.target!=object.id || track.muted {continue}
            let v=track.value_at(time,0.);
            match track.property.as_str() {
                "x"=>out.x=v,
                "y"=>out.y=v,
                "rotation"=>out.rotation=v,
                "opacity"=>out.opacity=v,
                "width"=>out.width=v,
                "height"=>out.height=v,
                "sx"=>out.transform3d.sx=v,
                "sy"=>out.transform3d.sy=v,
                "sz"=>out.transform3d.sz=v,
                "z"=>out.transform3d.z=v,
                "rx"=>out.transform3d.rx=v,
                "ry"=>out.transform3d.ry=v,
                "rz"=>out.transform3d.rz=v,
                _=>{}
            }
        }
        out
    }
    fn update(&mut self, message:Message)->iced::Task<Message> {
        match message {
            Message::Surface(s)=>{self.surface=s; self.menu_open=false; if s==Surface::Motion {self.status="Motion Lab / keyframe system".into();}},
            Message::Tool(t)=>self.tool=t,
            Message::SelectObject(id)=>{self.selected=Some(id);self.inspector=true;},
            Message::ToggleInspector=>self.inspector=!self.inspector,
            Message::InspectorSize(w)=>self.inspector_width=w.clamp(260,480) as u16,
            Message::PlayPause=>{self.playing=!self.playing;self.status=if self.playing{"Playing"}else{"Paused"}.into();},
            Message::Stop=>{self.playing=false;self.playhead=0.;},
            Message::Tick=>{
                let dt=1.0/60.0;
                if self.playing {                    self.playhead+=dt;
                    let duration=self.doc.scene().animation.duration;
                    if self.playhead>duration {
                        if self.doc.scene().animation.looped {self.playhead%=duration.max(0.001);}
                        else {self.playhead=duration;self.playing=false;}
                    }
                }
            },
            Message::SetPlayhead(v)=>{self.playhead=v.clamp(0.,self.doc.scene().animation.duration);},
            Message::AddKeyframe=>{
                if let Some(selected_id)=self.selected.clone() {
                    self.snapshot();
                    let t=self.playhead;
                    let current=self.doc.scene().objects.iter().find(|o|o.id==selected_id).cloned();
                    if let Some(o)=current {
                        let mut props=vec![("x",o.x),("y",o.y),("rotation",o.rotation),("opacity",o.opacity)];
                        if o.kind=="model3d" {props.extend([("z",o.transform3d.z),("rx",o.transform3d.rx),("ry",o.transform3d.ry),("rz",o.transform3d.rz),("sx",o.transform3d.sx),("sy",o.transform3d.sy),("sz",o.transform3d.sz)]);}
                        for (name,value) in props {
                            if let Some(track)=self.scene_mut().animation.tracks.iter_mut().find(|tr|tr.target==selected_id && tr.property==name) {
                                track.keyframes.push(Keyframe::new(t,value));
                            } else {
                                self.scene_mut().animation.tracks.push(Track{id:id("trk"),target:selected_id.clone(),property:name.into(),keyframes:vec![Keyframe::new(t,value)],muted:false,locked:false});
                            }
                        }
                    }
                    self.status=format!("Keyframes captured at {:.2}s",t);
                } else {self.status="Select an object first".into();}
            },
            Message::AddTrack(property)=>{
                if let Some(selected_id)=self.selected.clone() {
                    self.snapshot();
                    let playhead=self.playhead;
                    let value=self.doc.scene().objects.iter().find(|o|o.id==selected_id).map(|o|match property.as_str(){"x"=>o.x,"y"=>o.y,"rotation"=>o.rotation,"opacity"=>o.opacity,"z"=>o.transform3d.z,"rx"=>o.transform3d.rx,"ry"=>o.transform3d.ry,"rz"=>o.transform3d.rz,_=>0.}).unwrap_or(0.);
                    self.scene_mut().animation.tracks.push(Track{id:id("trk"),target:selected_id,property:property.clone(),keyframes:vec![Keyframe::new(playhead,value)],muted:false,locked:false});
                    self.status=format!("Track {} created",property);
                }
            },
            Message::SetEase(e)=>{
                let playhead=self.playhead;
                for track in &mut self.scene_mut().animation.tracks {
                    if let Some(k)=track.keyframes.iter_mut().min_by(|a,b|(a.time-playhead).abs().total_cmp(&(b.time-playhead).abs())) {k.easing=e;}
                }
            },
            Message::SetFps(v)=>self.scene_mut().animation.fps=v,
            Message::TextSize(v)=>{
                if let Some(o)=self.selected_object_mut(){ if o.kind=="text" {o.props["fontSize"]=json!(v);} }
            },
            Message::TextColor(c)=>{
                if let Some(o)=self.selected_object_mut(){ if o.kind=="text" {o.props["color"]=json!(c);} }
            },
            Message::TextAlign(a)=>{
                if let Some(o)=self.selected_object_mut(){ if o.kind=="text" {o.props["textAlign"]=json!(a);} }
            },
            Message::TextFormat(f)=>{
                if let Some(o)=self.selected_object_mut(){ if o.kind=="text" {o.props[&f]=json!(true);} }
            },
            Message::ToggleLoop(v)=>self.scene_mut().animation.looped=v,
            Message::ToggleOnion(v)=>self.scene_mut().animation.onion_skin=v,
            Message::Save=>{save_document(&self.doc);self.status="Saved to project storage".into();},
            Message::New=>{self.snapshot();self.doc=Document::sample();self.selected=None;self.playhead=0.;self.status="New SYN project".into();},
            Message::Duplicate=>{
                if let Some(selected_id)=self.selected.clone() {
                    self.snapshot();
                    if let Some(o)=self.doc.scene().objects.iter().find(|o|o.id==selected_id).cloned() {
                        let mut n=o;n.id=id("obj");n.x+=32.;n.y+=32.;self.scene_mut().objects.push(n);self.status="Object duplicated".into();
                    }
                }
            },
            Message::Delete=>{
                if let Some(id)=self.selected.take(){self.snapshot();self.scene_mut().objects.retain(|o|o.id!=id);self.scene_mut().animation.tracks.retain(|t|t.target!=id);self.status="Object deleted".into();}
            },
            Message::Undo=>{if let Some(prev)=self.history.pop(){self.future.push(self.doc.clone());self.doc=prev;}},
            Message::Redo=>{if let Some(next)=self.future.pop(){self.history.push(self.doc.clone());self.doc=next;}},
            Message::Validate=>{
                let mut errors=Vec::new();
                for s in &self.doc.scenes {
                    if s.width<=0.||s.height<=0.{errors.push(format!("Scene {} has invalid bounds",s.name));}
                    for tr in &s.animation.tracks {
                        if tr.keyframes.windows(2).any(|w|w[0].time>w[1].time){errors.push(format!("Track {} keyframes out of order",tr.property));}
                    }
                }
                self.status=if errors.is_empty(){"✓ SYN document validated: structure, scenes and animation tracks are coherent".into()}else{format!("Validation: {}",errors.join(" · "))};
            },
            Message::Export=>{
                let raw=serde_json::to_string_pretty(&self.doc).unwrap_or_default();
                download("syn-studio.syn",&raw);
                self.status="Exported .syn project".into();
            },
            Message::Preview=>self.overlay=Some("Preview".into()),
            Message::CodeEdit(action)=>self.code.perform(action),
            Message::ApplyCode=>{
                if let Ok(d)=serde_json::from_str::<Document>(&self.code.text()) {self.snapshot();self.doc=d;self.status="Source applied to live document".into();} else {self.status="Source rejected: invalid SYN JSON".into();}
            },
            Message::FormatCode=>{
                if let Ok(v)=serde_json::from_str::<Value>(&self.code.text()){self.code=text_editor::Content::with_text(&serde_json::to_string_pretty(&v).unwrap_or_default());self.status="Source formatted".into();}else{self.status="Format requires valid JSON".into();}
            },
            Message::Status(s)=>self.status=s,
            Message::Zoom(v)=>self.zoom=v,
            Message::TimelineZoom(v)=>self.timeline_zoom=v,
            Message::ToggleMenu=>self.menu_open=!self.menu_open,
            Message::CloseOverlay=>self.overlay=None,
            Message::OpenWorkspace(s)=>{self.overlay=Some(s);self.menu_open=false;},
        }
        iced::Task::none()
    }

    fn subscription(&self)->Subscription<Message> {
        if self.playing {
            time::every(Duration::from_millis(16)).map(|_| Message::Tick)
        } else { Subscription::none() }
    }

    fn view(&self)->Element<'_,Message> {
        if let Some(ref overlay)=self.overlay { return self.overlay_view(overlay); }

        let workspaces = [
            (Surface::Design, "Design"),
            (Surface::Motion, "Motion"),
            (Surface::Architecture, "System"),
            (Surface::Media, "Media"),
            (Surface::Code, "Code"),
            (Surface::Ai, "AI"),
            (Surface::Publish, "Publish"),
        ].into_iter().map(|(surface, label)| {
            button(text(label).size(12))
                .on_press(Message::Surface(surface))
                .padding([6, 9])
                .style(tab_style(surface == self.surface))
                .into()
        }).collect::<Vec<Element<'_, Message>>>();

        let workspace_nav = scrollable(
            iced::widget::Row::with_children(workspaces)
                .spacing(2)
                .align_y(alignment::Vertical::Center)
        )
        .direction(scrollable::Direction::Horizontal(scrollable::Scrollbar::hidden()))
        .width(Length::Shrink)
        .height(40);

        let right = if self.inspector {
            container(self.inspector_view())
                .width(Length::Fixed(self.inspector_width as f32))
                .height(Length::Fill)
        } else {
            container(space()).width(Length::Fixed(0.0)).height(Length::Fill)
        };

        let body = row![
            self.rail(),
            container(self.center()).width(Length::Fill).height(Length::Fill),
            right
        ].height(Length::Fill);

        let status = container(row![
            row![text("●").size(7).color(GOOD), text("Ready").size(11).color(TEXT)].spacing(6),
            text(&self.status).size(11).color(MUTED),
            space().width(Length::Fill),
            text(format!("{:.2}s", self.playhead)).size(10).color(MUTED),
            text(format!("{} fps", self.doc.scene().animation.fps)).size(10).color(MUTED),
        ].spacing(14).align_y(alignment::Vertical::Center))
        .height(30).padding([0, 16]).style(status_bar_style());

        column![
            self.titlebar(),
            container(workspace_nav).height(44).padding([0, 14]).style(chrome_style()),
            body,
            status
        ].height(Length::Fill).into()
    }

    fn titlebar(&self)->Element<'_,Message> {
        row![
            row![
                container(text("S").size(15).font(Font::MONOSPACE).color(BG))
                    .width(30).height(30).center(30).style(accent_mark_style()),
                column![
                    text("SYN STUDIO").size(14).font(Font::MONOSPACE).color(TEXT),
                    text("Creative Systems").size(9).color(MUTED),
                ].spacing(0),
                text(" / ").size(12).color(LINE),
                text(&self.doc.title).size(11).color(MUTED),
            ].spacing(9).align_y(alignment::Vertical::Center),
            space().width(Length::Fill),
            button(icon("M7 7h10M7 12h10M7 17h10"))
                .on_press(Message::OpenWorkspace("Command Menu".into()))
                .style(icon_button_style(false)).width(34).height(34),
            button(icon("M9 15 4 10l5-5 M4 10h11a5 5 0 0 1 5 5v1"))
                .on_press(Message::Undo)
                .style(icon_button_style(false)).width(34).height(34),
            button(icon("M15 15l5-5-5-5 M20 10H9a5 5 0 0 0-5 5v1"))
                .on_press(Message::Redo)
                .style(icon_button_style(false)).width(34).height(34),
            button(text("Save").size(11)).on_press(Message::Save)
                .style(text_button_style(false)).padding([7, 11]),
            button(text("Preview").size(11)).on_press(Message::Preview)
                .style(text_button_style(true)).padding([7, 13]),
        ].spacing(5).padding([9, 14]).height(52)
         .align_y(alignment::Vertical::Center).into()
    }

    fn ribbon(&self)->Element<'_,Message> {
        // Kept as a compatibility shim for older call sites.
        container(space()).height(0).into()
    }

    fn rail(&self)->Element<'_,Message> {
        let items = [
            (Tool::Select, "M4 4l16 8-7 2-2 7-7-17z"),
            (Tool::Draw, "M4 16l10-10 4 4-10 10H4v-4z"),
            (Tool::Text, "M5 5h14M12 5v14M8 19h8"),
            (Tool::Shape, "M12 3l9 9-9 9-9-9 9-9z"),
            (Tool::Camera, "M4 7h4l2-2h4l2 2h4v11H4V7z M9 12a3 3 0 1 0 6 0a3 3 0 0 0-6 0z"),
            (Tool::Bone, "M7 7a2.5 2.5 0 1 0-3.5-3.5M17 17a2.5 2.5 0 1 0 3.5 3.5M6 6l12 12"),
        ];
        let controls = items.into_iter().map(|(tool, path)| {
            button(icon(path)).on_press(Message::Tool(tool))
                .style(icon_button_style(tool == self.tool)).width(38).height(38).into()
        }).collect::<Vec<_>>();

        container(column![
            container(text("S").size(12).font(Font::MONOSPACE).color(ACCENT))
                .width(38).height(38).center(38).style(rail_mark_style()),
            iced::widget::Column::with_children(controls).spacing(4),
            space().height(Length::Fill),
            button(icon("M4 12h16M12 4v16"))
                .on_press(Message::OpenWorkspace("Insert".into()))
                .style(icon_button_style(false)).width(38).height(38),
        ].spacing(9).padding([10, 8]))
        .width(54).height(Length::Fill).style(rail_style()).into()
    }

    fn center(&self)->Element<'_,Message> {
        let title = match self.surface {
            Surface::Design => "Design", Surface::Motion => "Motion",
            Surface::Architecture => "System", Surface::Media => "Media",
            Surface::Code => "Code", Surface::Ai => "AI", Surface::Publish => "Publish",
        };

        let header = container(row![
            column![
                text(title).size(16).color(TEXT),
                text(self.surface_subtitle()).size(10).color(MUTED),
            ].spacing(2),
            space().width(Length::Fill),
            button(text(format!("{:.0}%", self.zoom * 100.0)).size(10))
                .on_press(Message::Zoom(self.zoom)).style(quiet_button_style(false)).padding([5, 7]),
            button(text("Fit").size(10)).on_press(Message::Zoom(0.72))
                .style(quiet_button_style(false)).padding([5, 7]),
            button(row![
                text(if self.inspector { "Inspector" } else { "Inspect" }).size(10),
                icon(if self.inspector { "M18 6L6 18M6 6l12 12" } else { "M4 12h16M4 6h16M4 18h16" })
            ].spacing(6).align_y(alignment::Vertical::Center))
                .on_press(Message::ToggleInspector)
                .style(quiet_button_style(self.inspector)).padding([6, 9]),
        ].spacing(7).align_y(alignment::Vertical::Center))
        .height(54).padding([0, 16]).style(workspace_header_style());

        match self.surface {
            Surface::Design | Surface::Motion => {
                let canvas_view = canvas(SceneCanvas {
                    scene: self.doc.scene().clone(), selected: self.selected.clone(),
                    playhead: self.playhead, zoom: self.zoom
                }).width(Length::Fill).height(Length::Fill);

                let stage = container(canvas_view).width(Length::Fill)
                    .height(Length::Fill).style(content_style());

                let controls = if self.surface == Surface::Motion {
                    row![
                        button(text(if self.playing { "Pause" } else { "Play" }).size(11))
                            .on_press(Message::PlayPause).style(text_button_style(self.playing)).padding([7, 11]),
                        button(text("Stop").size(11)).on_press(Message::Stop).style(quiet_button_style(false)).padding([7, 10]),
                        button(text("Keyframe").size(11)).on_press(Message::AddKeyframe).style(quiet_button_style(false)).padding([7, 10]),
                        space().width(Length::Fill),
                        button(text(if self.doc.scene().animation.looped { "Loop" } else { "Loop off" }).size(10))
                            .on_press(Message::ToggleLoop(!self.doc.scene().animation.looped))
                            .style(quiet_button_style(self.doc.scene().animation.looped)).padding([6, 9]),
                        button(text("Onion").size(10))
                            .on_press(Message::ToggleOnion(!self.doc.scene().animation.onion_skin))
                            .style(quiet_button_style(self.doc.scene().animation.onion_skin)).padding([6, 9]),
                    ]
                } else {
                    row![
                        button(text("New").size(11)).on_press(Message::New).style(quiet_button_style(false)).padding([7, 10]),
                        button(text("Duplicate").size(11)).on_press(Message::Duplicate).style(quiet_button_style(false)).padding([7, 10]),
                        button(text("Delete").size(11)).on_press(Message::Delete).style(quiet_button_style(false)).padding([7, 10]),
                        space().width(Length::Fill),
                        button(text("−").size(14)).on_press(Message::Zoom((self.zoom - 0.1).max(0.25))).style(quiet_button_style(false)).padding([5, 8]),
                        text(format!("{:.0}%", self.zoom * 100.0)).size(10).color(MUTED),
                        button(text("+").size(14)).on_press(Message::Zoom((self.zoom + 0.1).min(2.5))).style(quiet_button_style(false)).padding([5, 8]),
                    ]
                };

                let contextual = if self.surface == Surface::Motion {
                    column![controls, self.timeline()]
                } else {
                    column![controls]
                };

                column![
                    header,
                    container(stage).padding([8, 14]).height(Length::Fill),
                    container(contextual).padding([10, 14])
                ].spacing(0).height(Length::Fill).into()
            }
            Surface::Architecture => self.architecture_surface(),
            Surface::Media => self.media_surface(),
            Surface::Code => self.code_surface(),
            Surface::Ai => self.ai_surface(),
            Surface::Publish => self.publish_surface(),
        }
    }

    fn surface_name(&self)->String {
        match self.surface {
            Surface::Design=>"Command Center".into(),
            Surface::Motion=>"Motion Lab".into(),
            Surface::Architecture=>"Architecture".into(),
            Surface::Media=>"Media Lab".into(),
            Surface::Code=>"Code Intelligence".into(),
            Surface::Ai=>"AI Workbench".into(),
            Surface::Publish=>"Publish & Validate".into(),
        }
    }
    fn surface_subtitle(&self)->String {
        match self.surface {
            Surface::Design=>"SCENE / LIVE RUNTIME".into(),
            Surface::Motion=>"NON-LINEAR KEYFRAME TIMELINE".into(),
            Surface::Architecture=>"DOCUMENT / SYSTEM GRAPH".into(),
            Surface::Media=>"ASSET PIPELINE / PORTABLE MEDIA".into(),
            Surface::Code=>"RUST SOURCE / SYN DOCUMENT".into(),
            Surface::Ai=>"ASSISTED WORKFLOWS / SAFE OPERATIONS".into(),
            Surface::Publish=>"VALIDATION / PACKAGING / PREVIEW".into(),
        }
    }
    fn code_surface(&self)->Element<'_,Message> {
        let editor = text_editor(&self.code).placeholder("Edit the SYN document…")
            .highlight("json", iced::highlighter::Theme::Base16Mocha)
            .on_action(Message::CodeEdit).padding(16).size(13).height(Length::Fill);
        container(column![
            row![
                column![text("SYN source").size(12).color(TEXT), text(format!("{} lines", self.code.line_count())).size(10).color(MUTED)].spacing(2),
                space().width(Length::Fill),
                button(text("Format").size(10)).on_press(Message::FormatCode).style(quiet_button_style(false)).padding([6, 9]),
                button(text("Validate").size(10)).on_press(Message::Validate).style(quiet_button_style(false)).padding([6, 9]),
                button(text("Apply").size(10)).on_press(Message::ApplyCode).style(text_button_style(true)).padding([6, 10]),
            ].spacing(6).align_y(alignment::Vertical::Center),
            container(editor).padding([10, 0]).height(Length::Fill),
        ].spacing(4).padding([18, 18]))
        .height(Length::Fill).style(content_style()).into()
    }

    fn architecture_surface(&self)->Element<'_,Message> {
        let rows = [
            ("State", "Typed document, scene, object and animation state."),
            ("Render", "Iced Canvas owns the visual surface and interaction."),
            ("Motion", "Keyframes, easing and playback stay in Rust."),
            ("3D", "Transform3D supports position, rotation and scale."),
            ("Input", "Canvas events become typed application messages."),
            ("Async", "Playback uses Iced subscriptions, not a browser loop."),
        ];
        let content = rows.into_iter().map(|(name, description)| {
            container(row![
                text(name).size(12).color(TEXT).width(90),
                text(description).size(11).color(MUTED),
                space().width(Length::Fill),
            ].spacing(18).align_y(alignment::Vertical::Center))
            .padding([13, 2]).style(list_row_style()).into()
        }).collect::<Vec<_>>();
        column![
            scrollable(column![
                text("System overview").size(22).color(TEXT),
                text("The runtime is deliberately boring: state is explicit, rendering is native, and the document stays portable.")
                    .size(12).color(MUTED),
                iced::widget::Column::with_children(content).spacing(0),
            ].spacing(10).padding([22, 28]))
        ].height(Length::Fill).into()
    }

    fn media_surface(&self)->Element<'_,Message> {
        container(column![
            text("Media").size(22).color(TEXT),
            text("Assets stay close to the scene without turning the workspace into a dashboard.").size(12).color(MUTED),
            row![
                media_card("Image", "Raster / SVG"),
                media_card("Video", "Playback source"),
                media_card("Audio", "Timeline source"),
                media_card("3D", "Model asset"),
            ].spacing(12),
            container(row![
                column![
                    text("Current project").size(10).color(MUTED),
                    text(format!("{} scenes · {} objects · {} tracks", self.doc.scenes.len(), self.doc.scene().objects.len(), self.doc.scene().animation.tracks.len())).size(15).color(TEXT),
                ].spacing(4),
                space().width(Length::Fill),
                button(text("Insert").size(11)).on_press(Message::OpenWorkspace("Insert Media".into())).style(text_button_style(true)).padding([7, 11]),
            ].align_y(alignment::Vertical::Center)).padding([14, 0]).style(content_rule_style()),
        ].spacing(12).padding([22, 28])).height(Length::Fill).into()
    }

    fn ai_surface(&self)->Element<'_,Message> {
        container(column![
            text("AI WORKBENCH").size(9).color(ACCENT),
            text("AI can operate on typed project state, not opaque UI guesses.").size(24).color(TEXT),
            text("The workbench is intentionally separated from the renderer so generated changes can be validated before they touch the live document.").size(10).color(MUTED),
            row![
                workspace_button("Analyze document","Inspect scenes, tracks and structural issues",Message::Validate),
                workspace_button("Generate layout","Prepare a structured scene operation",Message::OpenWorkspace("Layout Generator".into())),
                workspace_button("Motion assist","Open the keyframe workflow",Message::Surface(Surface::Motion)),
            ].spacing(10),
        ].spacing(14).padding(18)).height(Length::Fill).style(content_style()).into()
    }
    fn publish_surface(&self)->Element<'_,Message> {
        let valid=self.doc.scenes.iter().all(|s|s.width>0. && s.height>0.);
        container(column![
            text("PUBLISH").size(9).color(ACCENT),
            text("Validate. Preview. Export.").size(24).color(TEXT),
            container(row![text(if valid{"✓"}else{"!"}).size(18).color(if valid{GOOD}else{PINK}),text(if valid{"Document is structurally valid"}else{"Document needs attention"}).size(11).color(TEXT)]).padding(14).style(content_rule_style()),
            row![
                workspace_button("Validate","Run structural and animation checks",Message::Validate),
                workspace_button("Preview","Run the current scene at the current playhead",Message::Preview),
                workspace_button("Export .syn","Write the portable project document",Message::Export),
            ].spacing(10),
        ].spacing(14).padding(18)).height(Length::Fill).style(panel_style(BG)).into()
    }

    fn scene_tabs(&self)->Element<'_,Message> {
        row![
            text("Scene").size(10).color(MUTED),
            text(self.doc.scene().name.clone()).size(11).color(TEXT),
        ].spacing(8).align_y(alignment::Vertical::Center).into()
    }

    fn timeline(&self)->Element<'_,Message> {
        let duration = self.doc.scene().animation.duration;
        let tracks = &self.doc.scene().animation.tracks;
        let mut labels = column![text("Tracks").size(10).color(MUTED)].spacing(0);
        for track in tracks.iter().take(7) {
            labels = labels.push(
                container(row![
                    text(track.property.clone()).size(10).color(TEXT),
                    space().width(Length::Fill),
                    text(track.keyframes.len().to_string()).size(9).color(MUTED),
                ].align_y(alignment::Vertical::Center))
                .height(28).padding([0, 8]).style(list_row_style())
            );
        }
        let timeline = column![
            row![
                button(text(if self.playing { "Pause" } else { "Play" }).size(10)).on_press(Message::PlayPause).style(text_button_style(self.playing)).padding([6, 9]),
                button(text("Stop").size(10)).on_press(Message::Stop).style(quiet_button_style(false)).padding([6, 9]),
                text(format!("{:.2}s", self.playhead)).size(10).color(TEXT),
                space().width(Length::Fill),
                text("FPS").size(9).color(MUTED),
                pick_list(vec![24u32, 30, 60, 120], Some(self.doc.scene().animation.fps), Message::SetFps).text_size(10),
            ].spacing(7).align_y(alignment::Vertical::Center),
            slider(0.0..=duration, self.playhead, Message::SetPlayhead).step(1.0 / self.doc.scene().animation.fps.max(1) as f32),
            timeline_canvas(tracks, self.playhead, duration, self.timeline_zoom),
        ].spacing(5);
        container(row![
            container(labels).width(170),
            container(timeline).width(Length::Fill),
        ]).padding([10, 0]).height(184).style(timeline_style()).into()
    }

    fn inspector_view(&self)->Element<'_,Message> {
        let selected = self.selected.as_ref().and_then(|id| self.doc.scene().objects.iter().find(|o| &o.id == id));
        let mut body = column![
            row![
                column![text("Inspector").size(16).color(TEXT), text("Properties").size(10).color(MUTED)].spacing(2),
                space().width(Length::Fill),
                button(icon("M6 6l12 12M18 6L6 18")).on_press(Message::ToggleInspector)
                    .style(icon_button_style(false)).width(32).height(32),
            ].spacing(6).align_y(alignment::Vertical::Center),
        ].spacing(18).padding([18, 16]);

        if let Some(o) = selected {
            body = body.push(column![text(o.label.clone()).size(14).color(TEXT), text(format!("{} · {}", o.kind, o.id)).size(10).color(MUTED)].spacing(3));
            body = body.push(text("Transform").size(10).color(MUTED));
            body = body.push(row![value_box("X", o.x), value_box("Y", o.y)].spacing(8));
            body = body.push(row![value_box("W", o.width), value_box("H", o.height)].spacing(8));
            body = body.push(row![value_box("Rotation", o.rotation), value_box("Opacity", o.opacity)].spacing(8));
            if o.kind == "model3d" {
                body = body.push(text("3D").size(10).color(MUTED));
                body = body.push(row![value_box("RX", o.transform3d.rx), value_box("RY", o.transform3d.ry), value_box("RZ", o.transform3d.rz)].spacing(8));
                body = body.push(row![value_box("SX", o.transform3d.sx), value_box("SY", o.transform3d.sy), value_box("SZ", o.transform3d.sz)].spacing(8));
            }
            if o.kind == "text" {
                let fs = o.props.get("fontSize").and_then(Value::as_f64).unwrap_or(16.0) as f32;
                body = body.push(text("Typography").size(10).color(MUTED));
                body = body.push(row![
                    button(text("B").size(10)).on_press(Message::TextFormat("bold".into())).style(quiet_button_style(o.props.get("bold").and_then(Value::as_bool).unwrap_or(false))),
                    button(text("I").size(10)).on_press(Message::TextFormat("italic".into())).style(quiet_button_style(o.props.get("italic").and_then(Value::as_bool).unwrap_or(false))),
                    button(text("U").size(10)).on_press(Message::TextFormat("underline".into())).style(quiet_button_style(o.props.get("underline").and_then(Value::as_bool).unwrap_or(false))),
                ].spacing(4));
                body = body.push(row![
                    text("Size").size(10).color(MUTED),
                    slider(8.0..=96.0, fs, Message::TextSize).width(Length::Fill),
                    text(format!("{:.0}px", fs)).size(10).color(TEXT),
                ].spacing(8));
            }
            body = body.push(text("Animation").size(10).color(MUTED));
            body = body.push(row![
                button(text("Capture keyframe").size(10)).on_press(Message::AddKeyframe).style(text_button_style(true)).padding([7, 9]),
                button(text("+ Track").size(10)).on_press(Message::AddTrack("x".into())).style(quiet_button_style(false)).padding([7, 9]),
            ].spacing(6));
        } else {
            body = body.push(column![
                text("Nothing selected").size(14).color(TEXT),
                text("Select an object on the canvas to inspect its properties.").size(11).color(MUTED),
            ].spacing(6));
        }
        body = body.push(space().height(Length::Fill));
        body = body.push(text("Panel width").size(10).color(MUTED));
        body = body.push(slider(260.0..=360.0, self.inspector_width as f32, |v| Message::InspectorSize(v as i32)));
        container(scrollable(body)).width(Length::Fill).height(Length::Fill).style(inspector_style()).into()
    }

    fn overlay_view(&self,name:&str)->Element<'_,Message> {
        let overlay_name=name.to_string();
        let content=match name {
            "Preview"=>column![
                text("LIVE PREVIEW").size(9).color(ACCENT),
                text("SYN Runtime Preview").size(28).color(TEXT),
                text(format!("Frame {:.2}s  ·  {} objects",self.playhead,self.doc.scene().objects.len())).size(11).color(MUTED),
                container(canvas(SceneCanvas{scene:self.doc.scene().clone(),selected:None,playhead:self.playhead,zoom:0.75}).width(Length::Fill).height(Length::Fill)).padding(10).style(panel_style(BG)),
            ],
            "Document Model"=>column![text("DOCUMENT MODEL").size(9).color(ACCENT),text("Rust state → SYN serialization → canvas/runtime").size(18).color(TEXT),text(serde_json::to_string_pretty(&self.doc).unwrap_or_default()).size(9).color(MUTED)],
            other=>column![text("WORKSPACE").size(9).color(ACCENT),text(overlay_name).size(24).color(TEXT),text("This surface is wired to the same Rust state machine. It is an executable workspace, not a decorative dead button.").size(11).color(MUTED)],
        };
        container(column![
            row![text("SYN Studio").size(12).color(TEXT),space().width(Length::Fill),button(text("Close").size(10)).on_press(Message::CloseOverlay).style(quiet_button_style(false))].padding(12),
            container(content).padding(22).width(Length::Fill).height(Length::Fill),
        ]).width(Length::Fill).height(Length::Fill).style(panel_style(BG)).into()
    }
}

fn panel_style(bg:Color)->impl Fn(&Theme)->iced::widget::container::Style {
    move |_theme| iced::widget::container::Style{text_color:Some(TEXT),background:Some(bg.into()),border:border::rounded(7).color(rgb(0x202328)).width(1.0),shadow:Default::default(),snap:true}
}
fn stage_style()->impl Fn(&Theme)->iced::widget::container::Style { content_style() }
fn btn_style(active:bool)->impl Fn(&Theme,button::Status)->button::Style { quiet_button_style(active) }

fn icon<'a>(path:&'a str)->Element<'a,Message> {
    let svg = format!(r##"<svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="#a8a0d9" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"><path d="{}"/></svg>"##, path);
    iced::widget::svg(iced::widget::svg::Handle::from_memory(svg.into_bytes())).width(17).height(17).into()
}
fn media_card<'a>(kind:&'a str,desc:&'a str)->Element<'a,Message> {
    button(column![text(kind).size(12).color(TEXT), text(desc).size(10).color(MUTED)].spacing(4).align_x(alignment::Horizontal::Left))
        .on_press(Message::OpenWorkspace(format!("Insert {}", kind))).style(workspace_row_style()).padding([14, 2]).width(Length::Fill).into()
}
fn workspace_button<'a>(title:&'a str,desc:&'a str,msg:Message)->Element<'a,Message> {
    button(row![
        column![text(title).size(12).color(TEXT), text(desc).size(10).color(MUTED)].spacing(3),
        space().width(Length::Fill),
        icon("M9 18l6-6-6-6"),
    ].align_y(alignment::Vertical::Center))
    .on_press(msg).style(workspace_row_style()).padding([14, 2]).width(Length::Fill).into()
}
fn value_box(label:&str,value:f32)->Element<'_,Message> {
    container(column![text(label).size(9).color(MUTED), text(format!("{:.1}",value)).size(12).color(TEXT)].spacing(3))
        .padding([8, 0]).width(Length::Fill).style(field_style()).into()
}
fn tab_style(active:bool)->impl Fn(&Theme,button::Status)->button::Style {
    move |_theme,status| {
        let hover=matches!(status,button::Status::Hovered);
        button::Style{background:Some(if active{rgb(0x1b1a22).into()}else if hover{rgb(0x15171b).into()}else{Color::TRANSPARENT.into()}),text_color:if active{TEXT}else if hover{rgb(0xd7d9df)}else{MUTED},border:border::rounded(6).color(if active{rgb(0x2c2938)}else{Color::TRANSPARENT}).width(if active{1.0}else{0.0}),shadow:Default::default(),snap:true}
    }
}
fn icon_button_style(active:bool)->impl Fn(&Theme,button::Status)->button::Style {
    move |_theme,status| {
        let hover=matches!(status,button::Status::Hovered); let pressed=matches!(status,button::Status::Pressed);
        button::Style{background:Some(if active{rgb(0x26222f).into()}else if pressed{rgb(0x1b1d21).into()}else if hover{rgb(0x16181c).into()}else{Color::TRANSPARENT.into()}),text_color:TEXT,border:border::rounded(7).color(Color::TRANSPARENT).width(0.0),shadow:Default::default(),snap:true}
    }
}
fn quiet_button_style(active:bool)->impl Fn(&Theme,button::Status)->button::Style {
    move |_theme,status| {
        let hover=matches!(status,button::Status::Hovered);
        button::Style{background:Some(if active{rgb(0x24212d).into()}else if hover{rgb(0x17191d).into()}else{Color::TRANSPARENT.into()}),text_color:if active{TEXT}else if hover{TEXT}else{MUTED},border:border::rounded(6).color(Color::TRANSPARENT).width(0.0),shadow:Default::default(),snap:true}
    }
}
fn text_button_style(active:bool)->impl Fn(&Theme,button::Status)->button::Style {
    move |_theme,status| {
        let hover=matches!(status,button::Status::Hovered);
        button::Style{background:Some(if active{ACCENT.into()}else if hover{rgb(0x1a1c21).into()}else{Color::TRANSPARENT.into()}),text_color:if active{BG}else if hover{TEXT}else{MUTED},border:border::rounded(6).color(Color::TRANSPARENT).width(0.0),shadow:Default::default(),snap:true}
    }
}
fn rail_style()->impl Fn(&Theme)->iced::widget::container::Style {
    move |_theme| iced::widget::container::Style{text_color:Some(TEXT),background:Some(rgb(0x0d0f12).into()),border:border::rounded(0).color(rgb(0x202328)).width(1.0),shadow:Default::default(),snap:true}
}
fn rail_mark_style()->impl Fn(&Theme)->iced::widget::container::Style {
    move |_theme| iced::widget::container::Style{text_color:Some(ACCENT),background:Some(Color::TRANSPARENT.into()),border:border::rounded(8).color(Color::TRANSPARENT).width(0.0),shadow:Default::default(),snap:true}
}
fn accent_mark_style()->impl Fn(&Theme)->iced::widget::container::Style {
    move |_theme| iced::widget::container::Style{text_color:Some(BG),background:Some(ACCENT.into()),border:border::rounded(8).color(Color::TRANSPARENT).width(0.0),shadow:Default::default(),snap:true}
}
fn chrome_style()->impl Fn(&Theme)->iced::widget::container::Style {
    move |_theme| iced::widget::container::Style{text_color:Some(TEXT),background:Some(rgb(0x0d0f12).into()),border:border::rounded(0).color(Color::TRANSPARENT).width(0.0),shadow:Default::default(),snap:true}
}
fn workspace_header_style()->impl Fn(&Theme)->iced::widget::container::Style {
    move |_theme| iced::widget::container::Style{text_color:Some(TEXT),background:Some(Color::TRANSPARENT.into()),border:border::rounded(0).color(Color::TRANSPARENT).width(0.0),shadow:Default::default(),snap:true}
}
fn content_style()->impl Fn(&Theme)->iced::widget::container::Style {
    move |_theme| iced::widget::container::Style{text_color:Some(TEXT),background:Some(BG.into()),border:border::rounded(0).color(Color::TRANSPARENT).width(0.0),shadow:Default::default(),snap:true}
}
fn status_bar_style()->impl Fn(&Theme)->iced::widget::container::Style {
    move |_theme| iced::widget::container::Style{text_color:Some(TEXT),background:Some(rgb(0x0b0d10).into()),border:border::rounded(0).color(rgb(0x1e2126)).width(1.0),shadow:Default::default(),snap:true}
}
fn list_row_style()->impl Fn(&Theme)->iced::widget::container::Style {
    move |_theme| iced::widget::container::Style{text_color:Some(TEXT),background:Some(Color::TRANSPARENT.into()),border:border::rounded(0).color(rgb(0x24272c)).width(1.0),shadow:Default::default(),snap:true}
}
fn workspace_row_style()->impl Fn(&Theme,button::Status)->button::Style {
    move |_theme,status| {
        let hover=matches!(status,button::Status::Hovered);
        button::Style{background:Some(if hover{rgb(0x15171b).into()}else{Color::TRANSPARENT.into()}),text_color:if hover{TEXT}else{MUTED},border:border::rounded(6).color(Color::TRANSPARENT).width(0.0),shadow:Default::default(),snap:true}
    }
}
fn field_style()->impl Fn(&Theme)->iced::widget::container::Style {
    move |_theme| iced::widget::container::Style{text_color:Some(TEXT),background:Some(rgb(0x111317).into()),border:border::rounded(6).color(rgb(0x24272c)).width(1.0),shadow:Default::default(),snap:true}
}
fn timeline_style()->impl Fn(&Theme)->iced::widget::container::Style {
    move |_theme| iced::widget::container::Style{text_color:Some(TEXT),background:Some(rgb(0x101215).into()),border:border::rounded(8).color(rgb(0x202328)).width(1.0),shadow:Default::default(),snap:true}
}
fn inspector_style()->impl Fn(&Theme)->iced::widget::container::Style {
    move |_theme| iced::widget::container::Style{text_color:Some(TEXT),background:Some(rgb(0x101215).into()),border:border::rounded(0).color(rgb(0x26292e)).width(1.0),shadow:Default::default(),snap:true}
}
fn content_rule_style()->impl Fn(&Theme)->iced::widget::container::Style {
    move |_theme| iced::widget::container::Style{text_color:Some(TEXT),background:Some(Color::TRANSPARENT.into()),border:border::rounded(0).color(rgb(0x25282d)).width(1.0),shadow:Default::default(),snap:true}
}
struct CanvasGestureState {
    first: Option<(touch::Finger, Point)>,
    second: Option<(touch::Finger, Point)>,
    pinch_distance: Option<f32>,
}
impl Default for CanvasGestureState {
    fn default() -> Self { Self { first:None, second:None, pinch_distance:None } }
}
struct SceneCanvas { scene:Scene, selected:Option<String>, playhead:f32, zoom:f32 }
impl SceneCanvas {
    fn effective_scale(&self, bounds:Rectangle)->f32 {
        let available_w=(bounds.width-40.0).max(80.0);
        let available_h=(bounds.height-52.0).max(80.0);
        let fit=(available_w/self.scene.width).min(available_h/self.scene.height);
        (fit*(self.zoom/0.72)).clamp(0.08,2.5)
    }

    fn project3d(p:[f32;3], transform:Transform3D, center:Point)->Point {
        let (mut x,mut y,mut z)=(p[0]*transform.sx,p[1]*transform.sy,p[2]*transform.sz);
        let rx=transform.rx*PI/180.; let ry=transform.ry*PI/180.; let rz=transform.rz*PI/180.;
        let (sx,cx)=(rx.sin(),rx.cos());let (sy,cy)=(ry.sin(),ry.cos());let(sz,cz)=(rz.sin(),rz.cos());
        let y1=y*cx-z*sx; let z1=y*sx+z*cx; y=y1;z=z1;
        let x1=x*cy+z*sy; let z2=-x*sy+z*cy; x=x1;z=z2;
        let x2=x*cz-y*sz; let y2=x*sz+y*cz; x=x2;y=y2;
        let perspective=420.0/(420.0+z.max(-300.0));
        Point::new(center.x+x*perspective,center.y+y*perspective)
    }
}
impl Program<Message> for SceneCanvas {
    type State=CanvasGestureState;
    fn draw(&self,_state:&CanvasGestureState,renderer:&Renderer,_theme:&Theme,bounds:Rectangle,_cursor:mouse::Cursor)->Vec<Geometry> {
        let mut frame=Frame::new(renderer,bounds.size());
        frame.fill_rectangle(Point::ORIGIN,bounds.size(),rgb(0x090d14));
        let grid=Path::rectangle(Point::new(0.,0.),bounds.size());
        frame.stroke(&grid,Stroke{style:canvas::Style::Solid(LINE),width:1.,..Default::default()});
        let scale=self.effective_scale(bounds);
        let ox=(bounds.width-self.scene.width*scale)/2.;
        let oy=(bounds.height-self.scene.height*scale)/2.;
        let art=Path::rounded_rectangle(Point::new(ox,oy),Size::new(self.scene.width*scale,self.scene.height*scale),border::Radius::from(18.));
        frame.fill(&art,rgb(0x101722));
        frame.stroke(&art,Stroke{style:canvas::Style::Solid(rgb(0x344058)),width:1.,..Default::default()});
        for o0 in &self.scene.objects {
            if o0.hidden {continue;}
            let o=o0.clone();
            let x=ox+o.x*scale;let y=oy+o.y*scale;
            if o.kind=="model3d" {
                let center=Point::new(ox+o.transform3d.x*scale,oy+o.transform3d.y*scale);
                let s=70.*scale;
                let verts=[[-1.,-1.,-1.], [1.,-1.,-1.], [1.,1.,-1.], [-1.,1.,-1.], [-1.,-1.,1.], [1.,-1.,1.], [1.,1.,1.], [-1.,1.,1.]];
                let edges=[(0,1),(1,2),(2,3),(3,0),(4,5),(5,6),(6,7),(7,4),(0,4),(1,5),(2,6),(3,7)];
                for (a,b) in edges {frame.stroke(&Path::line(Self::project3d([verts[a][0]*s,verts[a][1]*s,verts[a][2]*s],o.transform3d,center),Self::project3d([verts[b][0]*s,verts[b][1]*s,verts[b][2]*s],o.transform3d,center)),Stroke{style:canvas::Style::Solid(PINK),width:2.,..Default::default()});}
                frame.fill_text(CanvasText{content:"3D MODEL".into(),position:Point::new(center.x-35.,center.y+90.),color:TEXT,size:iced::Pixels(10.0),..Default::default()});
                continue;
            }
            if o.kind=="drawing" {
                if o.points.len()>1 {
                    let mut b=canvas::path::Builder::new();
                    let p0=o.points[0];b.move_to(Point::new(x+p0[0]*scale,y+p0[1]*scale));
                    for p in &o.points[1..] {b.line_to(Point::new(x+p[0]*scale,y+p[1]*scale));}
                    frame.stroke(&b.build(),Stroke{style:canvas::Style::Solid(PINK),width:3.,..Default::default()});
                }
                continue;
            }
            let rect=Path::rounded_rectangle(Point::new(x,y),Size::new(o.width*scale,o.height*scale),border::Radius::from(14.));
            let color=if o.kind=="text"{rgb(0x151b2a)}else{rgb(0x1b2130)};
            frame.fill(&rect,color);
            frame.stroke(&rect,Stroke{style:canvas::Style::Solid(if self.selected.as_ref()==Some(&o.id){ACCENT}else{rgb(0x3a465f)}),width:if self.selected.as_ref()==Some(&o.id){2.}else{1.},..Default::default()});
            let label=if o.kind=="text"{"SYN STUDIO".to_string()}else{o.label.clone()};
            let text_size=o.props.get("fontSize").and_then(Value::as_f64).unwrap_or(14.0) as f32;
            let text_color=o.props.get("color").and_then(Value::as_str).map(parse_hex).unwrap_or(TEXT);
            let align=match o.props.get("textAlign").and_then(Value::as_str) {
                Some("center")=>alignment::Horizontal::Center,
                Some("right")=>alignment::Horizontal::Right,
                _=>alignment::Horizontal::Left,
            };
            frame.fill_text(CanvasText{content:label,position:Point::new(x+16.*scale,y+22.*scale),max_width:(o.width*scale-28.).max(40.),color:text_color,size:(text_size*scale).into(),align_x:align.into(),..Default::default()});
        }
        frame.fill_rectangle(Point::new(0.,bounds.height-28.),Size::new(bounds.width,28.),rgb(0x0c121b));
        frame.fill_text(CanvasText{content:format!("FRAME {:04}   /   {:.2}s", (self.playhead*60.) as u32,self.playhead),position:Point::new(12.,bounds.height-10.),color:MUTED,size:9.into(),..Default::default()});
        vec![frame.into_geometry()]
    }
    fn update(&self,state:&mut CanvasGestureState,event:&canvas::Event,bounds:Rectangle,cursor:mouse::Cursor)->Option<canvas::Action<Message>> {
        match event {
            canvas::Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                if let Some(p)=cursor.position_in(bounds) {
                    let scale=self.effective_scale(bounds);
                    let ox=(bounds.width-self.scene.width*scale)/2.;
                    let oy=(bounds.height-self.scene.height*scale)/2.;
                    for o in self.scene.objects.iter().rev() {
                        if o.kind=="model3d" {continue;}
                        let r=Rectangle{x:ox+o.x*scale,y:oy+o.y*scale,width:o.width*scale,height:o.height*scale};
                        if r.contains(p) {return Some(canvas::Action::publish(Message::SelectObject(o.id.clone())));}
                    }
                }
            }
            canvas::Event::Mouse(mouse::Event::WheelScrolled{delta}) => {
                let amount=match delta {
                    mouse::ScrollDelta::Lines{y,..} => *y,
                    mouse::ScrollDelta::Pixels{y,..} => *y/120.0,
                };
                if amount.abs()>0.001 {
                    return Some(canvas::Action::publish(Message::Zoom((self.zoom*(1.0+amount*0.08)).clamp(0.25,2.5))));
                }
            }
            canvas::Event::Touch(touch::Event::FingerPressed{id,position}) => {
                if state.first.is_none() { state.first=Some((*id,*position)); }
                else if state.second.is_none() { state.second=Some((*id,*position)); state.pinch_distance=state.first.zip(state.second).map(|(a,b)|distance(a.1,b.1)); }
            }
            canvas::Event::Touch(touch::Event::FingerMoved{id,position}) => {
                if let Some((fid,p))=state.first.as_mut() { if fid==id { *p=*position; } }
                if let Some((fid,p))=state.second.as_mut() { if fid==id { *p=*position; } }
                if let (Some(a),Some(b),Some(old))=(state.first,state.second,state.pinch_distance) {
                    let now=distance(a.1,b.1);
                    if old>2.0 && now>2.0 {
                        state.pinch_distance=Some(now);
                        return Some(canvas::Action::publish(Message::Zoom((self.zoom*(now/old)).clamp(0.25,2.5))));
                    }
                }
            }
            canvas::Event::Touch(touch::Event::FingerLifted{id,..}) | canvas::Event::Touch(touch::Event::FingerLost{id,..}) => {
                if state.first.map(|v|v.0)==Some(*id) { state.first=None; }
                if state.second.map(|v|v.0)==Some(*id) { state.second=None; }
                if state.first.is_none() || state.second.is_none() { state.pinch_distance=None; }
            }
            _=>{}
        }
        None
    }
}

fn distance(a:Point,b:Point)->f32 { ((a.x-b.x).powi(2)+(a.y-b.y).powi(2)).sqrt() }

fn timeline_canvas<'a>(tracks:&[Track],playhead:f32,duration:f32,zoom:f32)->Element<'a,Message> {
    let width=(720.*zoom).max(420.);
    let mut items=column![row![text("TIME").size(7).color(MUTED),space().width(Length::Fill),text(format!("{:.1}s",duration)).size(7).color(MUTED)].width(width)];
    for track in tracks.iter().take(5) {
        let mut r=row![text(format!("{:<10}",track.property)).size(8).color(MUTED).width(70)];
        let mut line=column![progress_bar(0.0..=duration,playhead)];
        for k in &track.keyframes {
            line=line.push(text(format!("◆ {:.1}",k.time)).size(7).color(if (k.time-playhead).abs()<0.05{PINK}else{ACCENT}));
        }
        r=r.push(line);
        items=items.push(r.height(28));
    }
    container(scrollable(items).direction(scrollable::Direction::Horizontal(scrollable::Scrollbar::default()))).height(120).width(Length::Fill).style(panel_style(BG)).into()
}

fn load_document()->Option<Document> {
    #[cfg(target_arch="wasm32")]
    {
        let window=web_sys::window()?;
        let storage=window.local_storage().ok()??;
        let raw=storage.get_item("syn-studio-document").ok()??;
        serde_json::from_str(&raw).ok()
    }
    #[cfg(not(target_arch="wasm32"))]
    { None }
}
fn save_document(doc:&Document) {
    let raw=serde_json::to_string(doc).unwrap_or_default();
    #[cfg(target_arch="wasm32")]
    if let Some(w)=web_sys::window() {
        if let Ok(Some(s))=w.local_storage() { let _=s.set_item("syn-studio-document",&raw); }
    }
    #[cfg(not(target_arch="wasm32"))]
    {
        let _=std::fs::write("syn-studio.syn",raw);
    }
}
fn download(name:&str,raw:&str) {
    #[cfg(target_arch="wasm32")]
    {
        let Some(w)=web_sys::window() else{return};
        let Some(doc)=w.document() else{return};
        let array=js_sys::Array::of1(&wasm_bindgen::JsValue::from_str(raw));
        let Ok(blob)=web_sys::Blob::new_with_str_sequence(&array) else{return};
        let Ok(url)=web_sys::Url::create_object_url_with_blob(&blob) else{return};
        let Ok(a)=doc.create_element("a") else{return};
        let Ok(a)=a.dyn_into::<web_sys::HtmlAnchorElement>() else{return};
        a.set_href(&url);a.set_download(name);a.click();
        let _=web_sys::Url::revoke_object_url(&url);
    }
    #[cfg(not(target_arch="wasm32"))]
    { let _=std::fs::write(name,raw); }
}

fn main() -> iced::Result {
    #[cfg(target_arch="wasm32")]
    console_error_panic_hook::set_once();
    #[cfg(target_arch="wasm32")]
    console_log::init_with_level(log::Level::Info).ok();
    iced::application(App::default, App::update, App::view)
        .subscription(App::subscription)
        .theme(Theme::Dark)
        .title("SYN Studio")
        .run()
}

#[cfg(target_arch="wasm32")]
#[wasm_bindgen(start)]
pub fn wasm_start() {
    let _ = main();
}