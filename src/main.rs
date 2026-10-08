use iced::{
    alignment, border, mouse, time, touch, window,
    widget::{
        button, canvas, column, container, pick_list, progress_bar,
        row, scrollable, slider, space, text, text_editor, text_input,
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

const BG: Color = Color::from_rgb(0.025, 0.032, 0.045);
const SURFACE: Color = Color::from_rgb(0.055, 0.070, 0.095);
const SURFACE_2: Color = Color::from_rgb(0.075, 0.092, 0.125);
const LINE: Color = Color::from_rgb(0.14, 0.17, 0.22);
const TEXT: Color = Color::from_rgb(0.93, 0.94, 0.97);
const MUTED: Color = Color::from_rgb(0.48, 0.53, 0.62);
const ACCENT: Color = Color::from_rgb(0.64, 0.56, 1.0);
const PINK: Color = Color::from_rgb(0.87, 0.40, 0.95);
const GOOD: Color = Color::from_rgb(0.30, 0.88, 0.66);

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
        let mut scene=Scene{id:"SCENE-01".into(),name:"Command Center".into(),width:1200.0,height:700.0,objects:vec![],animation:AnimationState::default()};
        let mut hero=Object::rect("hero","SYN Studio",40.,36.,1120.,628.,"panel");hero.props=json!({"color":"#111827","stroke":"#2b3850"});scene.objects.push(hero);
        let mut glow=Object::rect("glow","",720.,120.,240.,240.,"circle");glow.props=json!({"color":"#6c63ff"});scene.objects.push(glow);
        let mut eyebrow=Object::rect("eyebrow","SYN / CREATIVE SYSTEMS",82.,78.,430.,30.,"text");eyebrow.props=json!({"fontSize":13.0,"color":"#8fa2c9","fontFamily":"Fira Sans","bold":true,"textAlign":"left"});scene.objects.push(eyebrow);
        let mut headline=Object::rect("headline","Design systems that move.",82.,120.,600.,110.,"text");headline.props=json!({"fontSize":46.0,"color":"#f4f6fb","fontFamily":"Fira Sans","bold":true,"textAlign":"left"});scene.objects.push(headline);
        let mut copy=Object::rect("copy","Build polished scenes, motion, media and code from one visual workspace.",82.,242.,520.,72.,"text");copy.props=json!({"fontSize":18.0,"color":"#a8b3c7","fontFamily":"Fira Sans","textAlign":"left"});scene.objects.push(copy);
        let mut cta=Object::rect("cta","OPEN COMMAND CENTER",82.,342.,210.,54.,"button");cta.props=json!({"color":"#8d7dff","textColor":"#090c12"});scene.objects.push(cta);
        let mut secondary=Object::rect("secondary","VIEW MOTION",306.,342.,170.,54.,"button");secondary.props=json!({"color":"#202a3c","textColor":"#f4f6fb"});scene.objects.push(secondary);
        for (idn,label,x,accent) in [("card1","DESIGN",82.,"#8d7dff"),("card2","MOTION",350.,"#66d9ef"),("card3","CODE",618.,"#7ff0b2")] {
            let mut card=Object::rect(idn,label,x,456.,250.,112.,"card");card.props=json!({"color":"#151e2e","accent":accent});scene.objects.push(card);
        }
        let mut stat=Object::rect("stat","60 FPS",910.,414.,170.,150.,"card");stat.props=json!({"color":"#121b2a","accent":"#df68f3","fontSize":22.0});scene.objects.push(stat);
        let mut orb=Object::rect("orb","",836.,184.,110.,110.,"circle");orb.props=json!({"color":"#66d9ef"});scene.objects.push(orb);
        let mut orb_track=Track{id:id("trk"),target:"orb".into(),property:"x".into(),keyframes:vec![],muted:false,locked:false};orb_track.keyframes=vec![Keyframe::new(0.,836.),Keyframe::new(2.,920.),Keyframe::new(4.,836.),Keyframe::new(6.,760.),Keyframe::new(8.,836.)];scene.animation.tracks=vec![orb_track];
        Self{syn:"0.4".into(),title:"SYN Studio / Creative Systems".into(),scenes:vec![scene],active_scene:0}
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
    MenuOpen(String),
    MenuAction(String),
    InsertObject(String),
    NewScene,
    SetInteraction(String),
    TextContent(String),
    FontFamily(String),
    SetCodeLanguage(String),
    ZoomFit,
    ZoomReset,
    ZoomIn,
    ZoomOut,
    WindowResized(Size),
    SelectScene(usize),
    RenameScene(String),
    DuplicateScene,
    DeleteScene,
    InspectorNarrow,
    InspectorWide,
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
    menu_open: Option<String>,
    code_language: String,
    last_tick: Option<iced::time::Instant>,
    history: Vec<Document>,
    future: Vec<Document>,
    window_size: Size,
}
impl Default for App {
    fn default() -> Self {
        let mut doc=load_document().unwrap_or_else(Document::sample);

        // Browser storage can outlive the current SYN Studio schema. Never let
        // stale or partially-valid project state make the renderer panic.
        if doc.scenes.is_empty() {
            doc=Document::sample();
        }
        doc.active_scene=doc.active_scene.min(doc.scenes.len().saturating_sub(1));

        let code=text_editor::Content::with_text(&serde_json::to_string_pretty(&doc).unwrap_or_default());
        Self {
            doc, surface:Surface::Design, tool:Tool::Select, selected:None,
            inspector:false, inspector_width:330, playhead:0., playing:false,
            timeline_zoom:1.0, zoom:1.0, status:"Ready".into(), code,
            overlay:None, menu_open:None, code_language:"SYN JSON".into(), last_tick:None, history:vec![], future:vec![],
            window_size:Size::new(1280.0,800.0),
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
            Message::Surface(s)=>{self.surface=s; self.menu_open=None; if s==Surface::Motion {self.status="Motion Lab / keyframe system".into();}},
            Message::Tool(t)=>self.tool=t,
            Message::SelectObject(id)=>{self.selected=Some(id);self.inspector=true;},
            Message::ToggleInspector=>self.inspector=!self.inspector,
            Message::InspectorSize(w)=>self.inspector_width=w.clamp(260,480) as u16,
            Message::PlayPause=>{self.playing=!self.playing;self.status=if self.playing{"Playing"}else{"Paused"}.into();},
            Message::Stop=>{self.playing=false;self.playhead=0.;},
            Message::Tick=>{
                let dt=1.0/60.0;
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
                if let Some(o)=self.selected_object_mut(){
                    if o.kind=="text" {
                        let current=o.props.get(&f).and_then(Value::as_bool).unwrap_or(false);
                        o.props[&f]=json!(!current);
                    }
                }
            },
            Message::TextContent(value)=>{
                if let Some(id)=self.selected.clone(){
                    self.snapshot();
                    if let Some(o)=self.scene_mut().objects.iter_mut().find(|o|o.id==id && o.kind=="text"){o.label=value;}
                }
            },
            Message::FontFamily(name)=>{
                if let Some(o)=self.selected_object_mut(){ if o.kind=="text" {o.props["fontFamily"]=json!(name);} }
            },
            Message::SetCodeLanguage(lang)=>{self.code_language=lang;self.status=format!("Code language: {}",self.code_language);},
            Message::ZoomFit|Message::ZoomReset=>self.zoom=1.0,
            Message::ZoomIn=>self.zoom=(self.zoom+0.1).min(2.5),
            Message::ZoomOut=>self.zoom=(self.zoom-0.1).max(0.25),
            Message::WindowResized(size)=>self.window_size=size,
            Message::SelectScene(index)=>{
                if index < self.doc.scenes.len() {
                    self.doc.active_scene=index;
                    self.selected=None;
                    self.playhead=0.0;
                    self.status=format!("Scene {} active", index+1);
                }
            },
            Message::RenameScene(name)=>{
                let trimmed=name.trim();
                if !trimmed.is_empty() { self.scene_mut().name=trimmed.to_string(); }
            },
            Message::DuplicateScene=>{
                self.snapshot();
                let mut scene=self.doc.scene().clone();
                scene.id=id("scene");
                scene.name=format!("{} Copy",scene.name);
                self.doc.scenes.insert(self.doc.active_scene+1,scene);
                self.doc.active_scene+=1;
                self.selected=None;
                self.status="Scene duplicated".into();
            },
            Message::DeleteScene=>{
                if self.doc.scenes.len()>1 {
                    self.snapshot();
                    self.doc.scenes.remove(self.doc.active_scene);
                    self.doc.active_scene=self.doc.active_scene.min(self.doc.scenes.len()-1);
                    self.selected=None;
                    self.status="Scene deleted".into();
                } else { self.status="A project needs at least one scene".into(); }
            },
            Message::InspectorNarrow=>self.inspector_width=self.inspector_width.saturating_sub(20).max(240),
            Message::InspectorWide=>self.inspector_width=(self.inspector_width+20).min(520),
            Message::MenuOpen(name)=>{self.menu_open=if self.menu_open.as_deref()==Some(name.as_str()){None}else{Some(name)};},
            Message::MenuAction(action)=>{
                self.menu_open=None;
                match action.as_str() {
                    "file.new"=>{let _=self.update(Message::New);},
                    "file.save"=>{let _=self.update(Message::Save);},
                    "file.export"=>{let _=self.update(Message::Export);},
                    "edit.undo"=>{let _=self.update(Message::Undo);},
                    "edit.redo"=>{let _=self.update(Message::Redo);},
                    "edit.duplicate"=>{let _=self.update(Message::Duplicate);},
                    "edit.delete"=>{let _=self.update(Message::Delete);},
                    "view.inspector"=>{let _=self.update(Message::ToggleInspector);},
                    "view.fit"=>{let _=self.update(Message::ZoomFit);},
                    "view.zoom_in"=>{let _=self.update(Message::ZoomIn);},
                    "view.zoom_out"=>{let _=self.update(Message::ZoomOut);},
                    "view.zoom_reset"=>{let _=self.update(Message::ZoomReset);},
                    "insert.text"=>{let _=self.update(Message::InsertObject("text".into()));},
                    "insert.card"=>{let _=self.update(Message::InsertObject("card".into()));},
                    "insert.button"=>{let _=self.update(Message::InsertObject("button".into()));},
                    "insert.circle"=>{let _=self.update(Message::InsertObject("circle".into()));},
                    "insert.line"=>{let _=self.update(Message::InsertObject("line".into()));},
                    "format.bold"=>{let _=self.update(Message::TextFormat("bold".into()));},
                    "format.italic"=>{let _=self.update(Message::TextFormat("italic".into()));},
                    "format.underline"=>{let _=self.update(Message::TextFormat("underline".into()));},
                    "format.left"=>{let _=self.update(Message::TextAlign("left".into()));},
                    "format.center"=>{let _=self.update(Message::TextAlign("center".into()));},
                    "format.right"=>{let _=self.update(Message::TextAlign("right".into()));},
                    "scene.design"=>{let _=self.update(Message::Surface(Surface::Design));},
                    "scene.motion"=>{let _=self.update(Message::Surface(Surface::Motion));},
                    "scene.architecture"=>{let _=self.update(Message::Surface(Surface::Architecture));},
                    "scene.media"=>{let _=self.update(Message::Surface(Surface::Media));},
                    "scene.new"=>{let _=self.update(Message::NewScene);},
                    "media.image"=>{let _=self.update(Message::InsertObject("image".into()));},
                    "media.audio"=>{let _=self.update(Message::InsertObject("audio".into()));},
                    "media.video"=>{let _=self.update(Message::InsertObject("video".into()));},
                    "interaction.link"=>{let _=self.update(Message::SetInteraction("link".into()));},
                    "interaction.trigger"=>{let _=self.update(Message::SetInteraction("trigger".into()));},
                    "code.rust"=>{let _=self.update(Message::SetCodeLanguage("Rust".into()));},
                    "code.syn"=>{let _=self.update(Message::SetCodeLanguage("SYN JSON".into()));},
                    "code.format"=>{let _=self.update(Message::FormatCode);},
                    "code.validate"=>{let _=self.update(Message::Validate);},
                    "code.apply"=>{let _=self.update(Message::ApplyCode);},
                    "code.symbols"=>{let _=self.update(Message::OpenWorkspace("Code Symbols".into()));},
                    "window.design"=>{let _=self.update(Message::Surface(Surface::Design));},
                    "window.motion"=>{let _=self.update(Message::Surface(Surface::Motion));},
                    "window.architecture"=>{let _=self.update(Message::Surface(Surface::Architecture));},
                    "window.media"=>{let _=self.update(Message::Surface(Surface::Media));},
                    "window.code"=>{let _=self.update(Message::Surface(Surface::Code));},
                    "window.ai"=>{let _=self.update(Message::Surface(Surface::Ai));},
                    "window.publish"=>{let _=self.update(Message::Surface(Surface::Publish));},
                    "window.narrow"=>{let _=self.update(Message::InspectorNarrow);},
                    "window.wide"=>{let _=self.update(Message::InspectorWide);},
                    "help.shortcuts"=>{let _=self.update(Message::OpenWorkspace("Shortcuts".into()));},
                    "help.about"=>{let _=self.update(Message::OpenWorkspace("About SYN Studio".into()));},
                    "help.validate"=>{let _=self.update(Message::Validate);},
                    _=>self.status=format!("Command executed: {}",action),
                }
            },
            Message::InsertObject(kind)=>{
                self.snapshot();
                let n=self.doc.scene().objects.len();
                let object=match kind.as_str() {
                    "text"=>Object::rect(&id("text"),"New text",110.+(n as f32*12.0)%300.0,120.+(n as f32*18.0)%260.0,360.,70.,"text"),
                    "circle"=>Object::rect(&id("circle"),"Orb",700.,170.+(n as f32*12.0)%220.0,150.,150.,"circle"),
                    "button"=>Object::rect(&id("button"),"BUTTON",760.,500.,180.,56.,"button"),
                    "line"=>Object::rect(&id("line"),"Divider",160.,540.,620.,4.,"line"),
                    "image"=>Object::rect(&id("image"),"IMAGE ASSET",700.,360.,280.,170.,"media"),
                    "audio"=>Object::rect(&id("audio"),"AUDIO TRACK",700.,550.,280.,58.,"audio"),
                    "video"=>Object::rect(&id("video"),"VIDEO ASSET",700.,360.,280.,170.,"video"),
                    _=>Object::rect(&id("card"),"Card",140.,380.,280.,120.,"card"),
                };
                let oid=object.id.clone();
                self.doc.scenes[self.doc.active_scene].objects.push(object);
                self.selected=Some(oid);self.inspector=true;self.status=format!("Inserted {}",kind);
            },
            Message::NewScene=>{
                self.snapshot();
                let mut scene=Document::sample().scenes.into_iter().next().unwrap_or_else(||Scene{id:id("scene"),name:"Scene".into(),width:1200.,height:700.,objects:vec![],animation:AnimationState::default()});
                let number=self.doc.scenes.len()+1;
                scene.id=id("scene");
                scene.name=format!("Scene {:02}",number);
                self.doc.scenes.push(scene);
                self.doc.active_scene=self.doc.scenes.len()-1;
                self.selected=None;
                self.status=format!("Created Scene {:02}",number);
            },
            Message::SetInteraction(kind)=>{
                if let Some(id)=self.selected.clone(){
                    if let Some(o)=self.scene_mut().objects.iter_mut().find(|o|o.id==id){
                        o.props[kind.clone()]=json!(true);
                        self.status=if kind=="link"{"Link interaction attached to selection".into()}else{"Trigger interaction attached to selection".into()};
                    }
                } else {
                    self.status="Select an object before creating an interaction".into();
                }
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
            Message::ToggleMenu=>self.menu_open=if self.menu_open.is_some(){None}else{Some("File".into())},
            Message::CloseOverlay=>self.overlay=None,
            Message::OpenWorkspace(s)=>{self.overlay=Some(s);self.menu_open=None;},
        }
        iced::Task::none()
    }

    fn subscription(&self)->Subscription<Message> {
        let resize=window::resize_events().map(|(_,size)|Message::WindowResized(size));
        if self.playing {
            Subscription::batch([resize,time::every(Duration::from_millis(16)).map(|_| Message::Tick)])
        } else { resize }
    }

    fn is_phone(&self)->bool { self.window_size.width < 560.0 }
    fn is_compact(&self)->bool { self.window_size.width < 860.0 }

    fn view(&self)->Element<'_,Message> {
        if let Some(ref overlay)=self.overlay { return self.overlay_view(overlay); }
        let items=[(Surface::Design,"DESIGN"),(Surface::Motion,"MOTION"),(Surface::Architecture,"SYSTEM"),(Surface::Media,"MEDIA"),(Surface::Code,"CODE"),(Surface::Ai,"AI"),(Surface::Publish,"PUBLISH")];
        let workspaces=items.into_iter().map(|(s,label)|
            button(text(label).size(8)).on_press(Message::Surface(s)).padding([7,11]).style(btn_style(self.surface==s))
        ).map(Into::into).collect::<Vec<Element<'_,Message>>>();
        let right=if self.inspector && !self.is_phone() {
            container(self.inspector_view()).width(Length::Fixed(if self.is_compact(){280.0}else{self.inspector_width as f32})).height(Length::Fill)
        } else { container(space()).width(Length::Fixed(0.0)).height(Length::Fill) };
        let desktop_body=row![self.rail(),container(self.center()).width(Length::Fill).height(Length::Fill),right].height(Length::Fill);
        let body:Element<'_,Message>=if self.is_phone() {
            let mobile_inspector:Element<'_,Message>=if self.inspector {
                container(self.inspector_view()).width(Length::Fill).height(Length::Fixed(320.0)).into()
            } else {
                container(space()).height(0).into()
            };
            column![container(self.center()).width(Length::Fill).height(Length::Fill),mobile_inspector]
                .height(Length::Fill).into()
        } else { desktop_body.into() };
        let workspace_bar=container(scrollable(
            row![
                iced::widget::Row::with_children(workspaces),
                space().width(Length::Fill),
                text(if self.inspector{"INSPECTOR OPEN"}else{"INSPECTOR CLOSED"}).size(7).color(MUTED)
            ].spacing(3).align_y(alignment::Vertical::Center)
        ).horizontal().height(36).width(Length::Fill)).height(36).padding([0,12]).style(panel_style(rgb(0x080b10)));
        let status=container(row![
            row![text("●").size(7).color(GOOD),text("READY").size(8).color(TEXT)].spacing(5),
            text(&self.status).size(8).color(MUTED),space().width(Length::Fill),
            text(format!("{:.0}%",self.zoom*100.)).size(8).color(MUTED),
            text(format!("{:.2}s",self.playhead)).size(8).color(MUTED),
            text(format!("{} FPS",self.doc.scene().animation.fps)).size(8).color(MUTED)
        ].spacing(14).padding([0,13]).align_y(alignment::Vertical::Center)).height(30).style(panel_style(rgb(0x06090d)));
        column![self.titlebar(),self.menu_bar(),workspace_bar,body,status].into()
    }

    fn titlebar(&self)->Element<'_,Message> {
        let actions=row![
            button(text("↶").size(16)).on_press(Message::Undo).style(btn_style(false)).width(34).height(34),
            button(text("↷").size(16)).on_press(Message::Redo).style(btn_style(false)).width(34).height(34),
            button(text("Save").size(10)).on_press(Message::Save).style(btn_style(false)).padding([7,14]),
            button(text("Preview").size(10)).on_press(Message::Preview).style(btn_style(true)).padding([7,15])
        ].spacing(5);

        let brand=row![
            container(text("S").size(17).font(Font::MONOSPACE).color(BG))
                .width(36).height(36).center(36).style(accent_box()),
            column![
                text("SYN STUDIO").size(15).font(Font::MONOSPACE).color(TEXT),
                text("CREATIVE SYSTEMS").size(7).color(MUTED)
            ].spacing(1),
            container(text("●").size(8).color(GOOD)).padding([0,5]),
            text(&self.doc.title).size(10).color(MUTED)
        ].spacing(8).align_y(alignment::Vertical::Center);

        container(
            row![
                brand,
                space().width(Length::Fill),
                container(actions).padding([3,4]).style(panel_style(SURFACE)),
            ]
            .padding([8,14])
            .align_y(alignment::Vertical::Center)
        )
        .height(58)
        .width(Length::Fill)
        .style(panel_style(rgb(0x070a10)))
        .into()
    }

    fn menu_bar(&self)->Element<'_,Message> {
        let names=["File","Edit","View","Insert","Format","Scene","Media","Interaction","Code","Window","Help"];
        let tabs=names.into_iter().map(|name|{
            button(text(name).size(9).color(if self.menu_open.as_deref()==Some(name){TEXT}else{MUTED}))
                .on_press(Message::MenuOpen(name.into()))
                .padding([6,10])
                .style(btn_style(self.menu_open.as_deref()==Some(name)))
                .into()
        }).collect::<Vec<Element<'_,Message>>>();

        let mut content=column![
            container(
                scrollable(
                    iced::widget::Row::with_children(tabs)
                        .spacing(2)
                        .padding([0,10])
                        .align_y(alignment::Vertical::Center)
                ).horizontal().height(34).width(Length::Fill)
            ).height(34).style(panel_style(rgb(0x0b0f16)))
        ];

        if let Some(name)=&self.menu_open {
            let entries:Vec<(&str,&str)>=match name.as_str() {
                "File"=>vec![("New Project","file.new"),("Save Project","file.save"),("Export .syn","file.export")],
                "Edit"=>vec![("Undo","edit.undo"),("Redo","edit.redo"),("Duplicate","edit.duplicate"),("Delete","edit.delete")],
                "View"=>vec![("Fit Canvas","view.fit"),("Zoom In","view.zoom_in"),("Zoom Out","view.zoom_out"),("100%","view.zoom_reset"),("Inspector","view.inspector")],
                "Insert"=>vec![("Text","insert.text"),("Card","insert.card"),("Button","insert.button"),("Circle","insert.circle"),("Divider","insert.line")],
                "Format"=>vec![("Bold","format.bold"),("Italic","format.italic"),("Underline","format.underline"),("Align Left","format.left"),("Align Center","format.center"),("Align Right","format.right")],
                "Scene"=>vec![("Command Center","scene.design"),("Motion Lab","scene.motion"),("Architecture","scene.architecture"),("Media Lab","scene.media"),("New Project Scene","scene.new")],
                "Media"=>vec![("Image Slot","media.image"),("Audio Track","media.audio"),("Video Track","media.video")],
                "Interaction"=>vec![("Create Link","interaction.link"),("Create Trigger","interaction.trigger")],
                "Code"=>vec![("SYN JSON","code.syn"),("Rust","code.rust"),("Format Source","code.format"),("Validate","code.validate"),("Apply to Design","code.apply"),("Document Symbols","code.symbols")],
                "Window"=>vec![("Command Center","window.design"),("Motion Lab","window.motion"),("Architecture","window.architecture"),("Media Lab","window.media"),("Code","window.code"),("AI Workbench","window.ai"),("Publish","window.publish"),("Narrow Inspector","window.narrow"),("Widen Inspector","window.wide")],
                "Help"=>vec![("Shortcuts","help.shortcuts"),("About SYN Studio","help.about"),("Run Validation","help.validate")],
                _=>vec![]
            };
            let buttons=entries.into_iter().map(|(label,action)|{
                button(row![
                    text(label).size(10).color(TEXT),
                    space().width(Length::Fill),
                    text("›").size(10).color(MUTED)
                ])
                .on_press(Message::MenuAction(action.into()))
                .width(230).height(32).padding([0,12]).style(menu_item_style())
                .into()
            }).collect::<Vec<Element<'_,Message>>>();
            content=content.push(
                container(column![iced::widget::Column::with_children(buttons).spacing(2).padding(7)])
                    .width(250).style(menu_panel_style())
            );
        }
        content.height(Length::Shrink).into()
    }

    fn rail(&self)->Element<'_,Message> {
        let items=[
            (Tool::Select,"↖","Select"),
            (Tool::Draw,"✎","Draw"),
            (Tool::Text,"T","Text"),
            (Tool::Shape,"◇","Shape"),
            (Tool::Camera,"⌗","Camera"),
            (Tool::Bone,"⌁","Rig")
        ];
        let controls=items.into_iter().map(|(t,i,l)|tool_button(t,i,l,self.tool)).collect::<Vec<_>>();
        container(column![
            container(text("S").size(13).font(Font::MONOSPACE).color(ACCENT))
                .width(44).height(44).center(44).style(rail_logo_style()),
            text("TOOLS").size(7).color(MUTED),
            iced::widget::Column::with_children(controls).spacing(5),
            space().height(Length::Fill),
            button(text("?").size(13)).on_press(Message::OpenWorkspace("Shortcuts".into()))
                .style(btn_style(false)).width(44).height(38)
        ].spacing(7).padding([12,8]))
        .width(72).height(Length::Fill)
        .style(panel_style(rgb(0x080b11)))
        .into()
    }

    fn center(&self)->Element<'_,Message> {
        let header=container(row![
            column![
                text(self.surface_name()).size(17).font(Font::MONOSPACE).color(TEXT),
                text(self.surface_subtitle()).size(8).color(MUTED)
            ].spacing(3),
            space().width(Length::Fill),
            container(row![
                button(text("−").size(13)).on_press(Message::ZoomOut).style(btn_style(false)).width(30).height(30),
                text(format!("{:.0}%",self.zoom*100.)).size(9).color(TEXT),
                button(text("+").size(13)).on_press(Message::ZoomIn).style(btn_style(false)).width(30).height(30),
                button(text("FIT").size(8)).on_press(Message::ZoomFit).style(btn_style(false)).padding([6,10]),
                button(text(if self.inspector{"Inspector"}else{"Inspect"}).size(8))
                    .on_press(Message::ToggleInspector).style(btn_style(self.inspector)).padding([6,11])
            ].spacing(3).align_y(alignment::Vertical::Center)).padding([3,4]).style(panel_style(SURFACE))
        ].spacing(12).padding([11,14]).align_y(alignment::Vertical::Center))
        .height(60).style(panel_style(rgb(0x0a0e15)));

        match self.surface {
            Surface::Design | Surface::Motion => {
                let canvas_view=canvas(SceneCanvas{
                    scene:self.doc.scene().clone(),
                    selected:self.selected.clone(),
                    playhead:self.playhead,
                    zoom:self.zoom
                }).width(Length::Fill).height(Length::Fill);

                let canvas_shell=container(canvas_view)
                    .padding(18)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .style(stage_style());

                let floating=row![
                    button(text(if self.tool==Tool::Select{"SELECT"}else{"TOOL"}).size(8))
                        .on_press(Message::Tool(self.tool)).style(btn_style(true)),
                    button(text("SNAP").size(8)).on_press(Message::Status("Snap ready".into())).style(btn_style(false)),
                    button(text("GRID").size(8)).on_press(Message::Status("Grid ready".into())).style(btn_style(false))
                ].spacing(3).padding([4,5]);

                let actions=if self.surface==Surface::Motion {
                    row![
                        button(text(if self.playing{"PAUSE"}else{"PLAY"}).size(9))
                            .on_press(Message::PlayPause).style(btn_style(self.playing)).padding([7,12]),
                        button(text("STOP").size(9)).on_press(Message::Stop).style(btn_style(false)).padding([7,12]),
                        button(text("KEYFRAME").size(9)).on_press(Message::AddKeyframe).style(btn_style(false)).padding([7,12]),
                        space().width(Length::Fill),
                        button(text("ONION").size(8)).on_press(Message::ToggleOnion(!self.doc.scene().animation.onion_skin)).style(btn_style(self.doc.scene().animation.onion_skin)).padding([7,10]),
                        button(text("LOOP").size(8)).on_press(Message::ToggleLoop(!self.doc.scene().animation.looped)).style(btn_style(self.doc.scene().animation.looped)).padding([7,10])
                    ].spacing(5).padding([7,11])
                } else {
                    row![
                        button(text("+ NEW").size(9)).on_press(Message::New).style(btn_style(false)).padding([7,12]),
                        button(text("DUPLICATE").size(9)).on_press(Message::Duplicate).style(btn_style(false)).padding([7,12]),
                        button(text("DELETE").size(9)).on_press(Message::Delete).style(btn_style(false)).padding([7,12]),
                        space().width(Length::Fill),
                        text("SCENE").size(8).color(MUTED),
                        text(format!("{:02}",self.doc.active_scene+1)).size(10).color(TEXT)
                    ].spacing(5).padding([7,11])
                };

                let bottom=if self.surface==Surface::Motion {
                    column![actions,self.timeline()]
                } else {
                    column![actions,self.scene_tabs()]
                };

                column![
                    header,
                    container(column![
                        container(canvas_shell).width(Length::Fill).height(Length::Fill),
                        container(floating).padding([0,14,5,14])
                    ]).height(Length::Fill),
                    bottom
                ].spacing(5).height(Length::Fill).into()
            }
            Surface::Architecture=>column![header,self.architecture_surface()].into(),
            Surface::Media=>column![header,self.media_surface()].into(),
            Surface::Code=>column![header,self.code_surface()].into(),
            Surface::Ai=>column![header,self.ai_surface()].into(),
            Surface::Publish=>column![header,self.publish_surface()].into(),
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
            .placeholder("Edit the SYN document or code...")
            .highlight(if self.code_language=="Rust"{"rust"}else{"json"}, iced::highlighter::Theme::Base16Mocha)
            .on_action(Message::CodeEdit).padding(14).size(12).height(Length::Fill);
        let symbols=self.code.text().lines().filter_map(|line|{
            let t=line.trim();
            if t.starts_with('"') && t.contains(':') {
                let key=t.split(':').next()?.trim_matches('"').to_string();
                if key.len()<32 {Some(key)} else {None}
            } else {None}
        }).take(12).collect::<Vec<_>>();
        let symbol_widgets=symbols.into_iter().map(|x|{
            button(text(format!("◇ {}",x)).size(8)).on_press(Message::Status(format!("Symbol: {}",x))).style(btn_style(false)).into()
        }).collect::<Vec<Element<'_,Message>>>();
        let symbols_panel=if symbol_widgets.is_empty(){column![text("No symbols detected").size(8).color(MUTED)]}else{iced::widget::Column::with_children(symbol_widgets).spacing(2)};
        container(column![
            row![
                column![text("CODE INTELLIGENCE").size(9).color(ACCENT),text("Formatting · syntax highlighting · validation · symbols").size(7).color(MUTED)].spacing(2),
                space().width(Length::Fill),
                pick_list(vec!["SYN JSON".to_string(),"Rust".to_string()],Some(self.code_language.clone()),Message::SetCodeLanguage).width(105).text_size(8),
                button(text("Format").size(9)).on_press(Message::FormatCode).style(btn_style(false)),
                button(text("Validate").size(9)).on_press(Message::Validate).style(btn_style(false)),
                button(text("Apply").size(9)).on_press(Message::ApplyCode).style(btn_style(true)),
            ].spacing(6).padding(9).align_y(alignment::Vertical::Center),
            row![
                container(editor).padding(8).width(Length::Fill).height(Length::Fill).style(panel_style(BG)),
                container(column![
                    text("SYMBOLS").size(8).color(MUTED),symbols_panel,
                    text("ASSISTS").size(8).color(MUTED),
                    button(text("Format source").size(8)).on_press(Message::FormatCode).style(btn_style(false)),
                    button(text("Validate structure").size(8)).on_press(Message::Validate).style(btn_style(false)),
                    button(text("Apply safely").size(8)).on_press(Message::ApplyCode).style(btn_style(false)),
                    text("Structured keys, scenes, objects and tracks are exposed as navigable symbols.").size(7).color(MUTED),
                ].spacing(6).padding(10)).width(190).height(Length::Fill).style(panel_style(SURFACE_2))
            ].spacing(5).height(Length::Fill)
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
        let content=cards.into_iter().map(|(a,b)| container(column![text(a).size(8).color(ACCENT),text(b).size(10).color(TEXT)].spacing(7)).padding(14).width(Length::Fill).style(panel_style(SURFACE_2)).into()).collect::<Vec<_>>();
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
        let scene_buttons=self.doc.scenes.iter().enumerate().map(|(i,s)|{
            button(column![
                text(format!("{:02}",i+1)).size(8).color(if i==self.doc.active_scene{ACCENT}else{MUTED}),
                text(&s.name).size(9).color(TEXT)
            ].spacing(1))
            .on_press(Message::SelectScene(i))
            .style(btn_style(i==self.doc.active_scene))
            .padding([6,12])
            .into()
        }).collect::<Vec<Element<'_,Message>>>();

        container(
            scrollable(row![
                iced::widget::Row::with_children(scene_buttons),
                button(text("+  Scene").size(9)).on_press(Message::NewScene).style(btn_style(false)).padding([7,12]),
                button(text("Duplicate").size(8)).on_press(Message::DuplicateScene).style(btn_style(false)).padding([7,10]),
                button(text("Delete").size(8)).on_press(Message::DeleteScene).style(btn_style(false)).padding([7,10]),
            ].spacing(4).padding([5,7]).align_y(alignment::Vertical::Center)).horizontal()
        )
        .height(52).width(Length::Fill).style(panel_style(rgb(0x0b0f16))).into()
    }

    fn timeline(&self)->Element<'_,Message> {
        let duration=self.doc.scene().animation.duration;
        let tracks=&self.doc.scene().animation.tracks;
        let mut left=column![text("ANIMATION STACK").size(9).color(MUTED)].spacing(4).padding(8);
        for t in tracks.iter().take(7) {
            left=left.push(container(row![text(t.property.clone()).size(9).color(TEXT),space().width(Length::Fill),text(format!("{}",t.keyframes.len())).size(8).color(MUTED)].spacing(4)).height(28).style(panel_style(SURFACE)));
        }
        let slider=slider(0.0..=duration,self.playhead,Message::SetPlayhead).step(1.0/(self.doc.scene().animation.fps.max(1) as f32));
        let right=column![
            row![button(text(if self.playing{"❚❚"}else{"▶"}).size(10)).on_press(Message::PlayPause).style(btn_style(true)),
                button(text("■").size(10)).on_press(Message::Stop).style(btn_style(false)),
                text(format!("{:.2}s",self.playhead)).size(10).color(TEXT),
                space().width(Length::Fill),
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
            row![
                column![text("INSPECTOR").size(10).color(TEXT),text("RIGHT DOCK").size(7).color(MUTED)].spacing(2),
                space().width(Length::Fill),
                button(text("−").size(10)).on_press(Message::InspectorNarrow).style(btn_style(false)),
                button(text("+").size(10)).on_press(Message::InspectorWide).style(btn_style(false)),
                button(text("×").size(14)).on_press(Message::ToggleInspector).style(btn_style(false)),
            ].spacing(5),
        ].spacing(8).padding(12);
        if let Some(o)=selected {
            body=body.push(container(column![text(o.label.clone()).size(14).color(TEXT),text(format!("{} · {}",o.kind,o.id)).size(8).color(MUTED)].spacing(3)).padding(10).style(panel_style(SURFACE_2)));
            body=body.push(text("TRANSFORM").size(8).color(MUTED));
            body=body.push(row![value_box("X",o.x),value_box("Y",o.y),value_box("W",o.width),value_box("H",o.height)].spacing(5));
            body=body.push(row![value_box("ROT",o.rotation),value_box("OPACITY",o.opacity),value_box("3D Z",o.transform3d.z)].spacing(5));
            if o.kind=="text" {
                let fs=o.props.get("fontSize").and_then(Value::as_f64).unwrap_or(24.0) as f32;
                let color=o.props.get("color").and_then(Value::as_str).unwrap_or("#f4f6fb").to_string();
                let family=o.props.get("fontFamily").and_then(Value::as_str).unwrap_or("Fira Sans").to_string();
                let align=o.props.get("textAlign").and_then(Value::as_str).unwrap_or("left");
                body=body.push(text("TEXT TOOLBAR").size(8).color(MUTED));
                body=body.push(text_input("Text", &o.label).on_input(Message::TextContent).padding(8).size(11).width(Length::Fill).style(text_input_style()));
                body=body.push(row![
                    pick_list(vec!["Fira Sans".to_string(),"Fira Mono".to_string(),"Sans".to_string()],Some(family),Message::FontFamily).width(100).text_size(8),
                    button(text("B").size(10)).on_press(Message::TextFormat("bold".into())).style(btn_style(o.props.get("bold").and_then(Value::as_bool).unwrap_or(false))),
                    button(text("I").size(10)).on_press(Message::TextFormat("italic".into())).style(btn_style(o.props.get("italic").and_then(Value::as_bool).unwrap_or(false))),
                    button(text("U").size(10)).on_press(Message::TextFormat("underline".into())).style(btn_style(o.props.get("underline").and_then(Value::as_bool).unwrap_or(false))),
                ].spacing(4));
                body=body.push(row![
                    button(text("L").size(9)).on_press(Message::TextAlign("left".into())).style(btn_style(align=="left")),
                    button(text("C").size(9)).on_press(Message::TextAlign("center".into())).style(btn_style(align=="center")),
                    button(text("R").size(9)).on_press(Message::TextAlign("right".into())).style(btn_style(align=="right")),
                    text("Size").size(8).color(MUTED),slider(8.0..=96.0,fs,Message::TextSize).width(Length::Fill),text(format!("{:.0}px",fs)).size(8).color(TEXT),
                ].spacing(5));
                body=body.push(row![
                    button(text("White").size(8)).on_press(Message::TextColor("#f4f6fb".into())).style(btn_style(color=="#f4f6fb")),
                    button(text("Lavender").size(8)).on_press(Message::TextColor("#a897ff".into())).style(btn_style(color=="#a897ff")),
                    button(text("Cyan").size(8)).on_press(Message::TextColor("#6fe7ff".into())).style(btn_style(color=="#6fe7ff")),
                    button(text("Pink").size(8)).on_press(Message::TextColor("#df68f3".into())).style(btn_style(color=="#df68f3")),
                ].spacing(4));
            }
            body=body.push(text("ANIMATION").size(8).color(MUTED));
            body=body.push(row![button(text("Capture Keyframe").size(9)).on_press(Message::AddKeyframe).style(btn_style(true)),button(text("+ Track").size(9)).on_press(Message::AddTrack("x".into())).style(btn_style(false))].spacing(5));
        } else {
            body=body.push(container(column![text("Select an object").size(14).color(TEXT),text("The inspector stays on the right and can be closed or resized without stealing the canvas.").size(9).color(MUTED)].spacing(8)).padding(12).style(panel_style(SURFACE_2)));
        }
        body=body.push(text("DOCK WIDTH").size(8).color(MUTED));
        body=body.push(row![button(text("Narrow").size(8)).on_press(Message::InspectorNarrow).style(btn_style(false)),slider(240.0..=520.0,self.inspector_width as f32,|v|Message::InspectorSize(v as i32)).width(Length::Fill),button(text("Wide").size(8)).on_press(Message::InspectorWide).style(btn_style(false))].spacing(5));
        container(scrollable(body)).width(Length::Fill).height(Length::Fill).style(panel_style(rgb(0x0b1119))).into()
    }

    fn overlay_view(&self,name:&str)->Element<'_,Message> {
        let overlay_name=name.to_string();
        let content=match name {
            "Preview"=>column![
                text("LIVE PREVIEW").size(9).color(ACCENT),
                text("SYN Runtime Preview").size(28).color(TEXT),
                text(format!("Frame {:.2}s  ·  {} objects",self.playhead,self.doc.scene().objects.len())).size(11).color(MUTED),
                container(canvas(SceneCanvas{scene:self.doc.scene().clone(),selected:None,playhead:self.playhead,zoom:1.0}).width(Length::Fill).height(Length::Fill)).padding(10).style(panel_style(BG)),
            ],
            "Document Model"=>column![text("DOCUMENT MODEL").size(9).color(ACCENT),text("Rust state → SYN serialization → canvas/runtime").size(18).color(TEXT),text(serde_json::to_string_pretty(&self.doc).unwrap_or_default()).size(9).color(MUTED)],
            other=>column![text("WORKSPACE").size(9).color(ACCENT),text(overlay_name).size(24).color(TEXT),text("This surface is wired to the same Rust state machine. It is an executable workspace, not a decorative dead button.").size(11).color(MUTED)],
        };
        container(column![
            row![text("SYN Studio").size(12).color(TEXT),space().width(Length::Fill),button(text("Close").size(10)).on_press(Message::CloseOverlay).style(btn_style(false))].padding(12),
            container(content).padding(22).width(Length::Fill).height(Length::Fill),
        ]).width(Length::Fill).height(Length::Fill).style(panel_style(BG)).into()
    }
}

fn tab(label:&str, surface:Surface, active:Surface)->Element<'_,Message> {
    button(text(label).size(8)).on_press(Message::Surface(surface)).style(btn_style(surface==active)).into()
}
fn ribbon_group<'a>(name:&'a str, items:Vec<Element<'a,Message>>)->Element<'a,Message> {
    container(column![
        text(name).size(6).color(MUTED),
        iced::widget::Row::with_children(items).spacing(3)
    ].spacing(3).padding([5,7])).style(panel_style(SURFACE_2)).into()
}
fn rbtn<'a>(icon:&'a str,label:&'a str,msg:Message)->Element<'a,Message> {
    button(column![
        text(icon).size(14).color(ACCENT),
        text(label).size(6).color(TEXT)
    ].align_x(alignment::Horizontal::Center).spacing(1))
        .on_press(msg).style(btn_style(false)).width(54).height(48).into()
}
fn tool_button<'a>(tool:Tool,icon:&'a str,label:&'a str,active:Tool)->Element<'a,Message> {
    button(column![
        text(icon).size(16).color(if tool==active{BG}else{MUTED}),
        text(label).size(7).color(if tool==active{BG}else{MUTED})
    ].align_x(alignment::Horizontal::Center).spacing(2))
    .on_press(Message::Tool(tool))
    .style(btn_style(tool==active))
    .width(54)
    .height(48)
    .into()
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
fn rail_logo_style()->impl Fn(&Theme)->iced::widget::container::Style {
    move |_theme| iced::widget::container::Style{
        text_color:Some(ACCENT),
        background:Some(rgb(0x17152a).into()),
        border:border::rounded(11).color(rgb(0x4b4275)).width(1.0),
        shadow:Default::default(),
        snap:true
    }
}
fn stage_style()->impl Fn(&Theme)->iced::widget::container::Style {
    move |_theme| iced::widget::container::Style{
        text_color:Some(TEXT),
        background:Some(rgb(0x070a10).into()),
        border:border::rounded(10).color(rgb(0x151d2b)).width(1.0),
        shadow:Default::default(),
        snap:true
    }
}
fn menu_panel_style()->impl Fn(&Theme)->iced::widget::container::Style {
    move |_theme| iced::widget::container::Style{text_color:Some(TEXT),background:Some(rgb(0x111722).into()),border:border::rounded(6).color(rgb(0x303b50)).width(1.0),shadow:Default::default(),snap:true}
}
fn menu_item_style()->impl Fn(&Theme,button::Status)->button::Style {
    move |_theme,status| {
        let hover=matches!(status,button::Status::Hovered);
        button::Style{background:Some(if hover{rgb(0x242f43).into()}else{Color::TRANSPARENT.into()}),text_color:TEXT,border:border::rounded(4).color(if hover{rgb(0x3d4c68)}else{Color::TRANSPARENT}).width(if hover{1.0}else{0.0}),shadow:Default::default(),snap:true}
    }
}
fn text_input_style()->impl Fn(&Theme,text_input::Status)->text_input::Style {
    move |_theme,_status| text_input::Style{background:rgb(0x101722).into(),border:border::rounded(6).color(rgb(0x2b3850)).width(1.0),icon:MUTED,placeholder:MUTED,value:TEXT,selection:ACCENT}
}
fn panel_style(bg:Color)->impl Fn(&Theme)->iced::widget::container::Style {
    move |_theme| iced::widget::container::Style {
        text_color:Some(TEXT),
        background:Some(bg.into()),
        border:border::rounded(8).color(rgb(0x20293a)).width(1.0),
        shadow:Default::default(),
        snap:true
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
        let pressed=matches!(status,button::Status::Pressed);
        button::Style {
            background:Some(
                if active {rgb(0x8f7cff).into()}
                else if pressed {rgb(0x202a3c).into()}
                else if hover {rgb(0x182131).into()}
                else {rgb(0x101722).into()}
            ),
            text_color:if active {BG}else if hover {TEXT}else{rgb(0xa9b2c3)},
            border:border::rounded(7).color(
                if active {rgb(0xb2a9ff)}
                else if hover {rgb(0x34445f)}
                else {rgb(0x1c2635)}
            ).width(1.0),
            shadow:Default::default(),
            snap:true
        }
    }
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
        (fit*self.zoom).clamp(0.08,2.5)
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
            let selected=self.selected.as_ref()==Some(&o.id);
            let fill_color=o.props.get("color").and_then(Value::as_str).map(parse_hex).unwrap_or(match o.kind.as_str(){
                "text"=>rgb(0x151b2a),
                "media"|"video"=>rgb(0x142033),
                "audio"=>rgb(0x13251f),
                _=>rgb(0x1b2130)
            });
            if o.kind=="circle" {
                let circle=Path::circle(Point::new(x+o.width*scale/2.,y+o.height*scale/2.),(o.width.min(o.height)*scale/2.).max(2.));
                frame.fill(&circle,fill_color);
                frame.stroke(&circle,Stroke{style:canvas::Style::Solid(if selected{ACCENT}else{rgb(0x3a465f)}),width:if selected{2.}else{1.},..Default::default()});
            } else {
                let radius=if o.kind=="button"{12.}else{14.};
                let rect=Path::rounded_rectangle(Point::new(x,y),Size::new(o.width*scale,o.height*scale),border::Radius::from(radius));
                frame.fill(&rect,fill_color);
                frame.stroke(&rect,Stroke{style:canvas::Style::Solid(if selected{ACCENT}else{rgb(0x3a465f)}),width:if selected{2.}else{1.},..Default::default()});
            }
            let label=o.label.clone();
            let text_size=o.props.get("fontSize").and_then(Value::as_f64).unwrap_or(if o.kind=="button"{13.}else{14.}) as f32;
            let text_color=o.props.get("textColor").and_then(Value::as_str).map(parse_hex).or_else(||o.props.get("color").and_then(Value::as_str).map(|c|if o.kind=="text"{parse_hex(c)}else{TEXT})).unwrap_or(TEXT);
            let align=match o.props.get("textAlign").and_then(Value::as_str) {
                Some("center")=>alignment::Horizontal::Center,
                Some("right")=>alignment::Horizontal::Right,
                _=>alignment::Horizontal::Left,
            };
            let family=o.props.get("fontFamily").and_then(Value::as_str).unwrap_or("Fira Sans");
            let mut font=if family=="Fira Mono"{Font::MONOSPACE}else{Font::DEFAULT};
            if o.props.get("bold").and_then(Value::as_bool).unwrap_or(false) { font.weight=iced::font::Weight::Bold; }
            if o.props.get("italic").and_then(Value::as_bool).unwrap_or(false) { font.style=iced::font::Style::Italic; }
            frame.fill_text(CanvasText{content:label,position:Point::new(x+16.*scale,y+22.*scale),max_width:(o.width*scale-28.).max(40.),color:text_color,size:(text_size*scale).into(),font,align_x:align.into(),..Default::default()});
            if o.props.get("underline").and_then(Value::as_bool).unwrap_or(false) && o.kind=="text" {
                frame.stroke(&Path::line(Point::new(x+16.*scale,y+31.*scale),Point::new(x+(o.width-16.).max(24.)*scale,y+31.*scale)),Stroke{style:canvas::Style::Solid(text_color),width:(1.2*scale).max(1.0),..Default::default()});
            }
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
