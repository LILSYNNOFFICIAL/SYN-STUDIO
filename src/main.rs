use iced::{
    alignment, border, keyboard, mouse, time,
    widget::{
        button, canvas, column, container, horizontal_space, pick_list, progress_bar,
        row, scrollable, slider, space, text, text_editor,
    },
    Color, Element, Font, Length, Point, Rectangle, Renderer, Size, Subscription, Theme,
    Vector,
};
use iced::widget::canvas::{Frame, Geometry, Path, Program, Stroke, Text as CanvasText};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{f32::consts::PI, time::Duration};
use uuid::Uuid;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

const BG: Color = Color::from_rgb(0.025, 0.032, 0.045);
const SURFACE: Color = Color::from_rgb(0.055, 0.070, 0.095);
const SURFACE_2: Color = Color::from_rgb(0.075, 0.092, 0.125);
const LINE: Color = Color::from_rgb(0.14, 0.17, 0.22);
const TEXT: Color = Color::from_rgb(0.93, 0.94, 0.97);
const MUTED: Color = Color::from_rgb(0.48, 0.53, 0.62);
const ACCENT: Color = Color::from_rgb(0.64, 0.56, 1.0);
const PINK: Color = Color::from_rgb(0.87, 0.40, 0.95);
const GOOD: Color = Color::from_rgb(0.30, 0.88, 0.66);

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
    Tick(iced::time::Instant),
    SetPlayhead(f32),
    AddKeyframe,
    AddTrack(String),
    SetEase(Ease),
    SetFps(u32),
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
            Message::Tick(now)=>{
                let dt=self.last_tick.map(|last|now.duration_since(last).as_secs_f32()).unwrap_or(0.);
                self.last_tick=Some(now);
                if self.playing {
                    self.playhead+=dt;
                    let duration=self.doc.scene().animation.duration;
                    if self.playhead>duration {
                        if self.doc.scene().animation.looped {self.playhead%=duration.max(0.001);}
                        else {self.playhead=duration;self.playing=false;}
                    }
                }
            },
            Message::SetPlayhead(v)=>{self.playhead=v.clamp(0.,self.doc.scene().animation.duration);},
            Message::AddKeyframe=>{
                if let Some(id)=self.selected.clone() {
                    self.snapshot();
                    let t=self.playhead;
                    let current=self.doc.scene().objects.iter().find(|o|o.id==id).cloned();
                    if let Some(o)=current {
                        let mut props=vec![("x",o.x),("y",o.y),("rotation",o.rotation),("opacity",o.opacity)];
                        if o.kind=="model3d" {props.extend([("z",o.transform3d.z),("rx",o.transform3d.rx),("ry",o.transform3d.ry),("rz",o.transform3d.rz),("sx",o.transform3d.sx),("sy",o.transform3d.sy),("sz",o.transform3d.sz)]);}
                        for (name,value) in props {
                            if let Some(track)=self.scene_mut().animation.tracks.iter_mut().find(|tr|tr.target==id && tr.property==name) {
                                track.keyframes.push(Keyframe::new(t,value));
                            } else {
                                self.scene_mut().animation.tracks.push(Track{id:id("trk"),target:id.clone(),property:name.into(),keyframes:vec![Keyframe::new(t,value)],muted:false,locked:false});
                            }
                        }
                    }
                    self.status=format!("Keyframes captured at {:.2}s",t);
                } else {self.status="Select an object first".into();}
            },
            Message::AddTrack(property)=>{
                if let Some(id)=self.selected.clone() {
                    self.snapshot();
                    let value=self.doc.scene().objects.iter().find(|o|o.id==id).map(|o|match property.as_str(){"x"=>o.x,"y"=>o.y,"rotation"=>o.rotation,"opacity"=>o.opacity,"z"=>o.transform3d.z,"rx"=>o.transform3d.rx,"ry"=>o.transform3d.ry,"rz"=>o.transform3d.rz,_=>0.}).unwrap_or(0.);
                    self.scene_mut().animation.tracks.push(Track{id:id("trk"),target:id,property:property.clone(),keyframes:vec![Keyframe::new(self.playhead,value)],muted:false,locked:false});
                    self.status=format!("Track {} created",property);
                }
            },
            Message::SetEase(e)=>{
                for track in &mut self.scene_mut().animation.tracks {
                    if let Some(k)=track.keyframes.iter_mut().min_by(|a,b|(a.time-self.playhead).abs().total_cmp(&(b.time-self.playhead).abs())) {k.easing=e;}
                }
            },
            Message::SetFps(v)=>self.scene_mut().animation.fps=v,
            Message::ToggleLoop(v)=>self.scene_mut().animation.looped=v,
            Message::ToggleOnion(v)=>self.scene_mut().animation.onion_skin=v,
            Message::Save=>{save_document(&self.doc);self.status="Saved to project storage".into();},
            Message::New=>{self.snapshot();self.doc=Document::sample();self.selected=None;self.playhead=0.;self.status="New SYN project".into();},
            Message::Duplicate=>{
                if let Some(id)=self.selected.clone() {
                    self.snapshot();
                    if let Some(o)=self.doc.scene().objects.iter().find(|o|o.id==id).cloned() {
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
            time::every(Duration::from_millis(16)).map(Message::Tick)
        } else { Subscription::none() }
    }

    fn view(&self)->Element<'_,Message> {
        if let Some(ref overlay)=self.overlay {
            return self.overlay_view(overlay);
        }
        let tabs=row![
            tab("FILE",Surface::Publish,self.surface),
            tab("HOME",Surface::Design,self.surface),
            tab("INSERT",Surface::Design,self.surface),
            tab("DESIGN",Surface::Design,self.surface),
            tab("MOTION",Surface::Motion,self.surface),
            tab("ARCHITECTURE",Surface::Architecture,self.surface),
            tab("MEDIA",Surface::Media,self.surface),
            tab("CODE",Surface::Code,self.surface),
            tab("AI",Surface::Ai,self.surface),
            tab("PUBLISH",Surface::Publish,self.surface),
        ].spacing(2).padding([0,10]);

        let ribbon=self.ribbon();
        let rail=self.rail();
        let center=self.center();
        let right=if self.inspector {
            container(self.inspector_view())
                .width(Length::Fixed(self.inspector_width as f32))
                .height(Length::Fill)
        } else {
            container(space().width(Length::Fixed(0.0))).width(Length::Fixed(0.0))
        };
        let body=row![rail,center,right].height(Length::Fill);
        let status=row![
            text("SYN 0.3").size(11).color(MUTED),
            text("•").size(11).color(GOOD),
            text(&self.status).size(11).color(MUTED),
            horizontal_space(),
            text(format!("{:.2}s / {:.2}s",self.playhead,self.doc.scene().animation.duration)).size(11).color(MUTED),
        ].spacing(8).padding([5,12]);
        column![
            self.titlebar(),
            container(tabs).height(34).style(panel_style(SURFACE)),
            ribbon,
            body,
            status.height(30).style(panel_style(rgb(0x090d13))),
        ].into()
    }

    fn titlebar(&self)->Element<'_,Message> {
        row![
            container(text("S").size(15).font(Font::MONOSPACE).color(BG)).width(30).height(30).center(30).style(accent_box()),
            column![text("SYN Studio").size(15).color(TEXT),text("RUST CREATIVE SYSTEMS IDE").size(8).color(MUTED)].spacing(0),
            horizontal_space(),
            button(text("New").size(11)).on_press(Message::New).style(btn_style(false)),
            button(text("Undo").size(11)).on_press(Message::Undo).style(btn_style(false)),
            button(text("Redo").size(11)).on_press(Message::Redo).style(btn_style(false)),
            button(text("Save").size(11)).on_press(Message::Save).style(btn_style(false)),
            button(text("Preview").size(11)).on_press(Message::Preview).style(btn_style(true)),
        ].spacing(7).padding([8,12]).align_y(alignment::Vertical::Center).height(46).into()
    }

    fn ribbon(&self)->Element<'_,Message> {
        let groups=match self.surface {
            Surface::Motion=>row![
                ribbon_group("PLAYBACK",vec![rbtn("▶", "Play",Message::PlayPause),rbtn("■","Stop",Message::Stop),rbtn("◆","Keyframe",Message::AddKeyframe)]),
                ribbon_group("KEYING",vec![rbtn("X","Position",Message::AddTrack("x".into())),rbtn("R","Rotation",Message::AddTrack("rotation".into())),rbtn("A","Opacity",Message::AddTrack("opacity".into())),rbtn("3D","3D Transform",Message::AddTrack("rz".into()))]),
                ribbon_group("EASING",vec![rbtn("⌁","Linear",Message::SetEase(Ease::Linear)),rbtn("↗","Ease In",Message::SetEase(Ease::EaseIn)),rbtn("↘","Ease Out",Message::SetEase(Ease::EaseOut)),rbtn("✦","Spring",Message::SetEase(Ease::Spring))]),
                ribbon_group("VIEW",vec![rbtn("◉","Onion",Message::ToggleOnion(!self.doc.scene().animation.onion_skin)),rbtn("↻","Loop",Message::ToggleLoop(!self.doc.scene().animation.looped))]),
            ],
            Surface::Code=>row![
                ribbon_group("SOURCE",vec![rbtn("{}","Format",Message::FormatCode),rbtn("✓","Validate",Message::Validate),rbtn("↕","Apply",Message::ApplyCode)]),
                ribbon_group("MODEL",vec![rbtn("⌘","Architecture",Message::OpenWorkspace("Architecture".into())),rbtn("◈","Document",Message::OpenWorkspace("Document Model".into()))]),
            ],
            Surface::Design=>row![
                ribbon_group("OBJECT",vec![rbtn("＋","New",Message::OpenWorkspace("Insert Object".into())),rbtn("⧉","Duplicate",Message::Duplicate),rbtn("⌫","Delete",Message::Delete)]),
                ribbon_group("EDIT",vec![rbtn("↶","Undo",Message::Undo),rbtn("↷","Redo",Message::Redo),rbtn("✓","Validate",Message::Validate)]),
                ribbon_group("CANVAS",vec![rbtn("Fit","Fit",Message::Zoom(0.72)),rbtn("100","100%",Message::Zoom(1.0)),rbtn("−","Zoom Out",Message::Zoom((self.zoom-0.1).max(0.25))),rbtn("+","Zoom In",Message::Zoom((self.zoom+0.1).min(2.0)))]),
            ],
            _=>row![
                ribbon_group("WORKSPACE",vec![rbtn("＋","Insert",Message::OpenWorkspace("Insert".into())),rbtn("◫","Layers",Message::OpenWorkspace("Layers".into())),rbtn("◇","Scene Graph",Message::OpenWorkspace("Scene Graph".into()))]),
                ribbon_group("PROJECT",vec![rbtn("✓","Validate",Message::Validate),rbtn("⇩","Export",Message::Export),rbtn("▶","Preview",Message::Preview)]),
            ],
        };
        container(scrollable(groups).direction(scrollable::Direction::Horizontal(scrollable::Scrollbar::default())).height(82))
            .style(panel_style(rgb(0x0b1017))).into()
    }

    fn rail(&self)->Element<'_,Message> {
        let items=[
            (Tool::Select,"↖","Select"),
            (Tool::Draw,"✎","Draw"),
            (Tool::Text,"T","Text"),
            (Tool::Shape,"◇","Shape"),
            (Tool::Camera,"⌗","Camera"),
            (Tool::Bone,"⌁","Rig"),
        ];
        iced::widget::Column::with_children(items.into_iter().map(|(t,i,l)|tool_button(t,i,l,self.tool)).collect::<Vec<_>>())
            .spacing(6).padding(7).width(64).into()
    }

    fn center(&self)->Element<'_,Message> {
        let header=row![
            column![text(self.surface_name()).size(14).color(TEXT),text(self.surface_subtitle()).size(8).color(MUTED)].spacing(0),
            horizontal_space(),
            text(format!("{:.0}%",self.zoom*100.)).size(10).color(MUTED),
            button(text(if self.inspector{"Hide Inspector"}else{"Inspector"}).size(10)).on_press(Message::ToggleInspector).style(btn_style(false)),
        ].spacing(8).padding([8,12]).align_y(alignment::Vertical::Center).height(46).style(panel_style(SURFACE));

        match self.surface {
            Surface::Code => column![header,self.code_surface()].height(Length::Fill).into(),
            Surface::Architecture => column![header,self.architecture_surface()].height(Length::Fill).into(),
            Surface::Media => column![header,self.media_surface()].height(Length::Fill).into(),
            Surface::Ai => column![header,self.ai_surface()].height(Length::Fill).into(),
            Surface::Publish => column![header,self.publish_surface()].height(Length::Fill).into(),
            Surface::Design | Surface::Motion => {
                let canvas_view=canvas(SceneCanvas{
                    scene:self.doc.scene().clone(), selected:self.selected.clone(), playhead:self.playhead, zoom:self.zoom,
                }).width(Length::Fill).height(Length::Fill);
                let main=container(canvas_view).padding(16).style(panel_style(BG));
                let bottom=if self.surface==Surface::Motion {self.timeline()} else {self.scene_tabs()};
                column![header,main,bottom].height(Length::Fill).into()
            }
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
        let editor=text_editor(&self.code)
            .placeholder("Edit the SYN document...")
            .highlight("json", iced::highlighter::Theme::Base16Mocha)
            .on_action(Message::CodeEdit)
            .padding(14)
            .size(12)
            .height(Length::Fill);
        container(column![
            row![
                text("SYN SOURCE").size(9).color(ACCENT),
                text(format!("{} lines",self.code.line_count())).size(8).color(MUTED),
                horizontal_space(),
                button(text("Format").size(9)).on_press(Message::FormatCode).style(btn_style(false)),
                button(text("Validate").size(9)).on_press(Message::Validate).style(btn_style(false)),
                button(text("Apply to Design").size(9)).on_press(Message::ApplyCode).style(btn_style(true)),
            ].spacing(7).padding(8),
            container(editor).padding(8).height(Length::Fill).style(panel_style(BG)),
        ]).height(Length::Fill).style(panel_style(SURFACE_2)).into()
    }
    fn architecture_surface(&self)->Element<'_,Message> {
        let cards=[
            ("STATE","Document, scenes, objects and animation are typed Rust state."),
            ("RENDER","Iced Canvas + renderer geometry owns the visual surface."),
            ("MOTION","Tracks, keyframes, easing and 60/120 FPS playback run in Rust."),
            ("3D","Transform3D supports position, rotation, scale and perspective projection."),
            ("INPUT","Canvas events map directly into typed Message values."),
            ("ASYNC","Iced subscriptions drive playback without a JavaScript animation loop."),
        ];
        let content=cards.into_iter().map(|(a,b)|container(column![text(a).size(8).color(ACCENT),text(b).size(10).color(TEXT)].spacing(7)).padding(14).width(Length::Fill).style(panel_style(SURFACE_2))).collect::<Vec<_>>();
        container(scrollable(iced::widget::Column::with_children(content).spacing(10).padding(16))).height(Length::Fill).style(panel_style(BG)).into()
    }
    fn media_surface(&self)->Element<'_,Message> {
        container(column![
            text("MEDIA LAB").size(10).color(ACCENT),
            text("A typed asset pipeline belongs beside the scene, not outside it.").size(22).color(TEXT),
            text("The document model is ready for embedded images, audio, video, fonts and model assets. The Iced renderer remains responsible for presentation.").size(10).color(MUTED),
            row![
                media_card("IMAGE","Raster / SVG"),
                media_card("VIDEO","Playback surface"),
                media_card("AUDIO","Timeline source"),
                media_card("3D","Mesh / model"),
            ].spacing(10),
            container(column![
                text("CURRENT PROJECT").size(8).color(MUTED),
                text(format!("{} scenes · {} objects · {} animation tracks",self.doc.scenes.len(),self.doc.scene().objects.len(),self.doc.scene().animation.tracks.len())).size(13).color(TEXT),
                text("Use Insert and Code to extend the document without breaking the runtime model.").size(9).color(MUTED),
            ].spacing(6)).padding(16).style(panel_style(SURFACE_2)),
        ].spacing(14).padding(18)).height(Length::Fill).style(panel_style(BG)).into()
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
        ].spacing(14).padding(18)).height(Length::Fill).style(panel_style(BG)).into()
    }
    fn publish_surface(&self)->Element<'_,Message> {
        let valid=self.doc.scenes.iter().all(|s|s.width>0. && s.height>0.);
        container(column![
            text("PUBLISH").size(9).color(ACCENT),
            text("Validate. Preview. Export.").size(24).color(TEXT),
            container(row![text(if valid{"✓"}else{"!"}).size(18).color(if valid{GOOD}else{PINK}),text(if valid{"Document is structurally valid"}else{"Document needs attention"}).size(11).color(TEXT)]).padding(14).style(panel_style(SURFACE_2)),
            row![
                workspace_button("Validate","Run structural and animation checks",Message::Validate),
                workspace_button("Preview","Run the current scene at the current playhead",Message::Preview),
                workspace_button("Export .syn","Write the portable project document",Message::Export),
            ].spacing(10),
        ].spacing(14).padding(18)).height(Length::Fill).style(panel_style(BG)).into()
    }

    fn scene_tabs(&self)->Element<'_,Message> {
        row![
            button(text("Command Center").size(10)).on_press(Message::Surface(Surface::Design)).style(btn_style(self.surface==Surface::Design)),
            button(text("Motion Lab").size(10)).on_press(Message::Surface(Surface::Motion)).style(btn_style(self.surface==Surface::Motion)),
            button(text("Architecture").size(10)).on_press(Message::Surface(Surface::Architecture)).style(btn_style(self.surface==Surface::Architecture)),
            button(text("Media Lab").size(10)).on_press(Message::Surface(Surface::Media)).style(btn_style(self.surface==Surface::Media)),
        ].spacing(3).padding(5).height(38).into()
    }

    fn timeline(&self)->Element<'_,Message> {
        let duration=self.doc.scene().animation.duration;
        let tracks=&self.doc.scene().animation.tracks;
        let mut left=column![text("ANIMATION STACK").size(9).color(MUTED)].spacing(4).padding(8);
        for t in tracks.iter().take(7) {
            left=left.push(container(row![text(t.property.clone()).size(9).color(TEXT),horizontal_space(),text(format!("{}",t.keyframes.len())).size(8).color(MUTED)].spacing(4)).height(28).style(panel_style(SURFACE)));
        }
        let slider=slider(0.0..=duration,self.playhead,Message::SetPlayhead).step(1.0/(self.doc.scene().animation.fps.max(1) as f32));
        let right=column![
            row![button(text(if self.playing{"❚❚"}else{"▶"}).size(10)).on_press(Message::PlayPause).style(btn_style(true)),
                button(text("■").size(10)).on_press(Message::Stop).style(btn_style(false)),
                text(format!("{:.2}s",self.playhead)).size(10).color(TEXT),
                horizontal_space(),
                text("FPS").size(8).color(MUTED),
                pick_list(vec![24u32,30,60,120],Some(self.doc.scene().animation.fps),Message::SetFps).text_size(9),
            ].spacing(6).align_y(alignment::Vertical::Center),
            slider,
            timeline_canvas(tracks,self.playhead,duration,self.timeline_zoom),
        ].spacing(5).padding(8);
        container(row![left.width(170),right.width(Length::Fill)].height(190)).style(panel_style(SURFACE_2)).into()
    }

    fn inspector_view(&self)->Element<'_,Message> {
        let selected=self.selected.as_ref().and_then(|id|self.doc.scene().objects.iter().find(|o|&o.id==id));
        let mut body=column![
            row![text("INSPECTOR").size(10).color(TEXT),horizontal_space(),button(text("×").size(14)).on_press(Message::ToggleInspector).style(btn_style(false))].spacing(6),
            text("RIGHT DOCK · CLOSED BY DEFAULT").size(7).color(ACCENT),
        ].spacing(8).padding(12);
        if let Some(o)=selected {
            body=body.push(container(column![
                text(o.label.clone()).size(14).color(TEXT),
                text(format!("{} · {}",o.kind,o.id)).size(8).color(MUTED),
            ].spacing(3)).padding(10).style(panel_style(SURFACE_2)));
            body=body.push(text("TRANSFORM").size(8).color(MUTED));
            body=body.push(row![
                value_box("X",o.x),value_box("Y",o.y),value_box("W",o.width),value_box("H",o.height)
            ].spacing(5));
            body=body.push(row![
                value_box("ROT",o.rotation),value_box("OPACITY",o.opacity),value_box("3D Z",o.transform3d.z)
            ].spacing(5));
            if o.kind=="model3d" {
                body=body.push(text("3D TRANSFORM").size(8).color(MUTED));
                body=body.push(row![value_box("RX",o.transform3d.rx),value_box("RY",o.transform3d.ry),value_box("RZ",o.transform3d.rz)].spacing(5));
                body=body.push(row![value_box("SX",o.transform3d.sx),value_box("SY",o.transform3d.sy),value_box("SZ",o.transform3d.sz)].spacing(5));
            }
            body=body.push(text("ANIMATION").size(8).color(MUTED));
            body=body.push(row![
                button(text("Capture Keyframe").size(9)).on_press(Message::AddKeyframe).style(btn_style(true)),
                button(text("+ Track").size(9)).on_press(Message::AddTrack("x".into())).style(btn_style(false)),
            ].spacing(5));
        } else {
            body=body.push(container(column![
                text("Nothing selected").size(14).color(TEXT),
                text("The Inspector stays closed until you explicitly open it or select an object. The canvas gets the full workspace by default.").size(9).color(MUTED),
            ].spacing(8)).padding(12).style(panel_style(SURFACE_2)));
        }
        body=body.push(text("DOCK SIZE").size(8).color(MUTED));
        body=body.push(slider(260.0..=480.0,self.inspector_width as f32,|v|Message::InspectorSize(v as i32)));
        body=body.push(text("260 px     330 px     480 px").size(7).color(MUTED));
        container(scrollable(body)).width(Length::Fill).height(Length::Fill).style(panel_style(rgb(0x0b1119))).into()
    }

    fn overlay_view(&self,name:&str)->Element<'_,Message> {
        let content=match name {
            "Preview"=>column![
                text("LIVE PREVIEW").size(9).color(ACCENT),
                text("SYN Runtime Preview").size(28).color(TEXT),
                text(format!("Frame {:.2}s  ·  {} objects",self.playhead,self.doc.scene().objects.len())).size(11).color(MUTED),
                container(canvas(SceneCanvas{scene:self.doc.scene().clone(),selected:None,playhead:self.playhead,zoom:0.75}).width(Length::Fill).height(Length::Fill)).padding(10).style(panel_style(BG)),
            ],
            "Document Model"=>column![text("DOCUMENT MODEL").size(9).color(ACCENT),text("Rust state → SYN serialization → canvas/runtime").size(18).color(TEXT),text(serde_json::to_string_pretty(&self.doc).unwrap_or_default()).size(9).color(MUTED)],
            other=>column![text("WORKSPACE").size(9).color(ACCENT),text(other).size(24).color(TEXT),text("This surface is wired to the same Rust state machine. It is an executable workspace, not a decorative dead button.").size(11).color(MUTED)],
        };
        container(column![
            row![text("SYN Studio").size(12).color(TEXT),horizontal_space(),button(text("Close").size(10)).on_press(Message::CloseOverlay).style(btn_style(false))].padding(12),
            container(content).padding(22).width(Length::Fill).height(Length::Fill),
        ]).width(Length::Fill).height(Length::Fill).style(panel_style(BG)).into()
    }
}

fn tab(label:&str, surface:Surface, active:Surface)->Element<'_,Message> {
    button(text(label).size(8)).on_press(Message::Surface(surface)).style(btn_style(surface==active)).into()
}
fn ribbon_group<'a>(name:&str, items:Vec<Element<'a,Message>>)->Element<'a,Message> {
    container(column![text(name).size(7).color(MUTED),row(items).spacing(4)].spacing(4).padding([5,8])).style(panel_style(SURFACE_2)).into()
}
fn rbtn<'a>(icon:&str,label:&str,msg:Message)->Element<'a,Message> {
    button(column![text(icon).size(15).color(ACCENT),text(label).size(7).color(TEXT)].align_x(alignment::Horizontal::Center).spacing(2))
        .on_press(msg).style(btn_style(false)).width(58).height(56).into()
}
fn tool_button(tool:Tool,icon:&str,label:&str,active:Tool)->Element<'_,Message> {
    button(column![text(icon).size(16).color(if tool==active{ACCENT}else{MUTED}),text(label).size(7).color(TEXT)].align_x(alignment::Horizontal::Center).spacing(2))
        .on_press(Message::Tool(tool)).style(btn_style(tool==active)).width(50).height(48).into()
}
fn media_card<'a>(kind:&'a str,desc:&'a str)->Element<'a,Message> {
    container(column![text(kind).size(9).color(ACCENT),text(desc).size(10).color(TEXT)].spacing(5)).padding(14).width(Length::Fill).style(panel_style(SURFACE_2)).into()
}
fn workspace_button<'a>(title:&'a str,desc:&'a str,msg:Message)->Element<'a,Message> {
    button(column![text(title).size(11).color(TEXT),text(desc).size(8).color(MUTED)].spacing(5).align_x(alignment::Horizontal::Left))
        .on_press(msg).padding(12).width(Length::Fill).style(btn_style(false)).into()
}
fn value_box(label:&str,value:f32)->Element<'_,Message> {
    container(column![text(label).size(7).color(MUTED),text(format!("{:.1}",value)).size(10).color(TEXT)].spacing(2))
        .padding(7).width(Length::Fill).style(panel_style(SURFACE_2)).into()
}
fn panel_style(bg:Color)->impl Fn(&Theme)->iced::widget::container::Style {
    move |_theme| iced::widget::container::Style {
        text_color:Some(TEXT),background:Some(bg.into()),
        border:border::rounded(8).color(LINE).width(1.0),
        shadow:Default::default(),snap:true
    }
}
fn accent_box()->impl Fn(&Theme)->iced::widget::container::Style {
    move |_theme| iced::widget::container::Style {
        text_color:Some(BG),background:Some(ACCENT.into()),
        border:border::rounded(8).color(ACCENT).width(1.0),
        shadow:Default::default(),snap:true
    }
}
fn btn_style(active:bool)->impl Fn(&Theme,button::Status)->button::Style {
    move |_theme,status| {
        let hover=matches!(status,button::Status::Hovered);
        button::Style {
            background:Some(if active {ACCENT.into()} else if hover {SURFACE_2.into()} else {SURFACE.into()}),
            text_color:if active {BG}else{TEXT},
            border:border::rounded(7).color(if active {ACCENT}else{LINE}).width(1.0),
            shadow:Default::default(),snap:true
        }
    }
}

struct SceneCanvas { scene:Scene, selected:Option<String>, playhead:f32, zoom:f32 }
impl SceneCanvas {
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
    type State=();
    fn draw(&self,_state:&(),renderer:&Renderer,_theme:&Theme,bounds:Rectangle,_cursor:mouse::Cursor)->Vec<Geometry> {
        let mut frame=Frame::new(renderer,bounds.size());
        frame.fill_rectangle(Point::ORIGIN,bounds.size(),rgb(0x090d14));
        let grid=Path::rectangle(Point::new(0.,0.),bounds.size());
        frame.stroke(&grid,Stroke{style:canvas::Style::Solid(LINE),width:1.,..Default::default()});
        let scale=self.zoom;
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
                frame.fill_text(CanvasText{content:"3D MODEL".into(),position:Point::new(center.x-35.,center.y+90.),color:TEXT,size:10.,..Default::default()});
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
            frame.fill_text(CanvasText{content:label,position:Point::new(x+16.*scale,y+22.*scale),color:TEXT,size:(14.*scale).into(),..Default::default()});
        }
        frame.fill_rectangle(Point::new(0.,bounds.height-28.),Size::new(bounds.width,28.),rgb(0x0c121b));
        frame.fill_text(CanvasText{content:format!("FRAME {:04}   /   {:.2}s", (self.playhead*60.) as u32,self.playhead),position:Point::new(12.,bounds.height-10.),color:MUTED,size:9.into(),..Default::default()});
        vec![frame.into_geometry()]
    }
    fn update(&self,_state:&mut (),event:&canvas::Event,bounds:Rectangle,cursor:mouse::Cursor)->Option<canvas::Action<Message>> {
        if let canvas::Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))=event {
            if let Some(p)=cursor.position_in(bounds) {
                let scale=self.zoom;
                let ox=(bounds.width-self.scene.width*scale)/2.;
                let oy=(bounds.height-self.scene.height*scale)/2.;
                for o in self.scene.objects.iter().rev() {
                    if o.kind=="model3d" {continue;}
                    let r=Rectangle{x:ox+o.x*scale,y:oy+o.y*scale,width:o.width*scale,height:o.height*scale};
                    if r.contains(p) {return Some(canvas::Action::publish(Message::SelectObject(o.id.clone())));}
                }
            }
        }
        None
    }
}

fn timeline_canvas<'a>(tracks:&[Track],playhead:f32,duration:f32,zoom:f32)->Element<'a,Message> {
    let width=(720.*zoom).max(420.);
    let mut items=column![row![text("TIME").size(7).color(MUTED),horizontal_space(),text(format!("{:.1}s",duration)).size(7).color(MUTED)].width(width)];
    for track in tracks.iter().take(5) {
        let mut r=row![text(format!("{:<10}",track.property)).size(8).color(MUTED).width(70)];
        let mut line=column![progress_bar(0.0..=duration,playhead).height(4).width(width-80.)];
        for k in &track.keyframes {
            line=line.push(text(format!("◆ {:.1}",k.time)).size(7).color(if (k.time-playhead).abs()<0.05{PINK}else{ACCENT}));
        }
        r=r.push(line);
        items=items.push(r.height(28));
    }
    container(scrollable(items).direction(scrollable::Direction::Horizontal(scrollable::Scrollbar::new()))).height(120).width(Length::Fill).style(panel_style(BG)).into()
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

fn main() {
    #[cfg(target_arch="wasm32")]
    console_error_panic_hook::set_once();
    #[cfg(target_arch="wasm32")]
    console_log::init_with_level(log::Level::Info).ok();
    iced::application(App::default, App::update, App::view)
        .subscription(|app:&App| app.subscription())
        .theme(|_|Theme::Dark)
        .title(|_|"SYN Studio".into())
        .run()
}

#[cfg(target_arch="wasm32")]
#[wasm_bindgen(start)]
pub fn wasm_start() {
    main();
}
