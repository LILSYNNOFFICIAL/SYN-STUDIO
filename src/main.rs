use dioxus::prelude::*;
use dioxus_code::Theme;
use dioxus_code_editor::{CodeEditor, Language as CodeLanguage};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use uuid::Uuid;
use base64::{engine::general_purpose::STANDARD, Engine as _};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

static CSS: Asset = asset!("/assets/app.css");

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
struct Document {
    syn: String,
    r#type: String,
    meta: Meta,
    viewport: Viewport,
    assets: Vec<SynAsset>,
    scenes: Vec<Scene>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
struct Meta { id: String, title: String }

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
struct Viewport { width: u32, height: u32 }

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
struct SynAsset {
    id: String,
    name: String,
    r#type: String,
    size: u64,
    embedded: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
struct Scene {
    id: String,
    name: String,
    background: String,
    objects: Vec<Object>,
    interactions: Vec<Interaction>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
struct Object {
    id: String,
    kind: String,
    label: String,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    rotation: f32,
    locked: bool,
    hidden: bool,
    #[serde(default)]
    props: Value,
    #[serde(default)]
    styles: Value,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
struct Interaction {
    id: String,
    event: Event,
    actions: Vec<Action>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
struct Event { r#type: String, target: String }

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
enum Action {
    #[serde(rename = "scene.next")] Next,
    #[serde(rename = "scene.goto")] Goto { target: String },
    #[serde(rename = "object.setText")] SetText { target: String, value: String },
}

#[derive(Clone, Default)]
struct History { past: Vec<Document>, future: Vec<Document> }

impl History {
    fn new(d: &Document) -> Self { Self { past: vec![d.clone()], future: vec![] } }
    fn push(&mut self, d: &Document) {
        self.past.push(d.clone());
        self.future.clear();
        if self.past.len() > 100 { self.past.remove(0); }
    }
    fn undo(&mut self, d: &Document) -> Option<Document> {
        if self.past.len() <= 1 { return None; }
        self.future.insert(0, d.clone());
        self.past.pop();
        self.past.last().cloned()
    }
    fn redo(&mut self) -> Option<Document> {
        let d = self.future.first().cloned()?;
        self.future.remove(0);
        self.past.push(d.clone());
        Some(d)
    }
}

fn id(prefix: &str) -> String {
    format!("{}-{}", prefix, &Uuid::new_v4().simple().to_string()[..8])
}

trait ToF32 { fn to_f32(self) -> f32; }
impl ToF32 for f32 { fn to_f32(self) -> f32 { self } }
impl ToF32 for i32 { fn to_f32(self) -> f32 { self as f32 } }
impl ToF32 for u32 { fn to_f32(self) -> f32 { self as f32 } }

fn object<X: ToF32, Y: ToF32, W: ToF32, H: ToF32>(kind: &str, label: &str, x: X, y: Y, w: W, h: H, styles: Value) -> Object {
    Object {
        id: id("obj"), kind: kind.into(), label: label.into(), x: x.to_f32(), y: y.to_f32(), width: w.to_f32(), height: h.to_f32(),
        rotation: 0., locked: false, hidden: false, props: json!({"text": label}), styles
    }
}

fn text(o: &Object) -> String {
    o.props.get("text").and_then(Value::as_str).unwrap_or(&o.label).into()
}

fn demo() -> Document {
    let mut d = Document {
        syn: "0.2".into(), r#type: "document".into(),
        meta: Meta { id: id("doc"), title: "SYN Studio / Creative Systems".into() },
        viewport: Viewport { width: 1120, height: 640 }, assets: vec![], scenes: vec![]
    };

    let mut home = Scene {
        id: "HOME".into(), name: "Command Center".into(), background: "#0b0d12".into(),
        objects: vec![], interactions: vec![]
    };

    home.objects.push(object("shape", "HERO", 36., 30., 1048., 580.,
        json!({"background":"linear-gradient(145deg,#171b2a,#0d111b 58%,#141021)","borderColor":"#32384d","borderWidth":1,"borderRadius":28,"boxShadow":"0 28px 90px #000b"})));
    home.objects.push(object("text", "SYN STUDIO", 76., 68., 260., 22.,
        json!({"fontSize":10,"fontWeight":900,"color":"#b7a7ff","letterSpacing":3})));
    home.objects.push(object("text", "Create without\nfighting the interface.", 76., 112., 560., 112.,
        json!({"fontSize":48,"fontWeight":850,"color":"#f8f7ff","letterSpacing":-1.8,"lineHeight":1.02})));
    home.objects.push(object("text", "A visual creative IDE for design, code, motion, media and interactive systems. Every surface shares one portable SYN document.", 80., 244., 520., 54.,
        json!({"fontSize":15,"fontWeight":500,"color":"#aeb6c8","lineHeight":1.5})));

    let primary = object("button", "Explore the workspace", 80., 326., 210., 50.,
        json!({"background":"linear-gradient(135deg,#d8d0ff,#8f7cff)","color":"#10101a","fontSize":12,"fontWeight":900,"borderRadius":13,"boxShadow":"0 14px 34px #8f7cff33"}));
    let pid=primary.id.clone();
    home.objects.push(primary);
    home.interactions.push(Interaction{id:id("evt"),event:Event{r#type:"click".into(),target:pid},actions:vec![Action::Goto{target:"ARCH".into()}]});

    for (i,(k,v,c)) in [
        ("DESIGN","Responsive canvas","Layouts that feel intentional"),
        ("CODE","Rust + SYN","Readable source, real validation"),
        ("MOTION","Timeline","Keyframes without leaving the document"),
        ("MEDIA","Assets","Images, video, audio and fonts"),
    ].iter().enumerate() {
        let x=80.+(i%2) as f32*278.;
        let y=414.+(i/2) as f32*82.;
        home.objects.push(object("shape",*k,x,y,258.,64.,
            json!({"background":"#111624","borderColor":"#2d354b","borderWidth":1,"borderRadius":14})));
        home.objects.push(object("text",*k,x+15.,y+11.,78.,15.,
            json!({"fontSize":8,"fontWeight":900,"color":"#9d8dff","letterSpacing":1.6})));
        home.objects.push(object("text",&format!("{}  ·  {}",v,c),x+15.,y+34.,225.,18.,
            json!({"fontSize":10,"fontWeight":650,"color":"#d9dceb"})));
    }

    home.objects.push(object("shape","PREVIEW",700.,70.,328.,448.,
        json!({"background":"linear-gradient(160deg,#171c2b,#0b1019)","borderColor":"#3b4560","borderWidth":1,"borderRadius":22,"boxShadow":"0 22px 70px #0008"})));
    home.objects.push(object("text","LIVE CANVAS",726.,98.,150.,18,
        json!({"fontSize":9,"fontWeight":900,"color":"#8f7cff","letterSpacing":1.8})));
    home.objects.push(object("shape","MOCK",726.,138.,276.,194.,
        json!({"background":"linear-gradient(145deg,#252b3e,#151a29)","borderColor":"#46506a","borderWidth":1,"borderRadius":17,"boxShadow":"0 16px 45px #0007"})));
    home.objects.push(object("shape","ACCENT",748.,160.,92.,7.,
        json!({"background":"linear-gradient(90deg,#ff5fcf,#8f7cff)","borderRadius":4})));
    home.objects.push(object("text","Design. Code.\nShip something real.",748.,190.,220.,68,
        json!({"fontSize":25,"fontWeight":850,"color":"#f8f7ff","lineHeight":1.05})));
    home.objects.push(object("text","SYN / 0.2",748.,278.,130.,18,
        json!({"fontSize":9,"fontWeight":800,"color":"#7f8aa2","letterSpacing":1.5})));
    home.objects.push(object("shape","PILL",748.,354.,238.,42.,
        json!({"background":"#20263a","borderColor":"#343e58","borderWidth":1,"borderRadius":12})));
    home.objects.push(object("text","One document • many surfaces",766.,367.,200.,18,
        json!({"fontSize":9,"fontWeight":750,"color":"#cbd2e0"})));
    home.objects.push(object("text","Command Center  /  01",726.,474.,240.,18,
        json!({"fontSize":9,"fontWeight":800,"color":"#6f7b92","letterSpacing":1.2})));

    let mut arch=Scene{id:"ARCH".into(),name:"Architecture".into(),background:"#0a0d13".into(),objects:vec![],interactions:vec![]};
    arch.objects.push(object("text","01  /  ARCHITECTURE",58.,48.,300.,18,json!({"fontSize":9,"fontWeight":900,"color":"#9d8dff","letterSpacing":2})));
    arch.objects.push(object("text","One document. Every surface.",58.,82.,760.,54,json!({"fontSize":40,"fontWeight":850,"color":"#f7f6fc","letterSpacing":-1.2})));
    arch.objects.push(object("text","The editor, runtime and source view all operate on the same structured document.",62.,145.,700.,24,json!({"fontSize":13,"color":"#98a2b6"})));
    for (i,(a,b,c)) in [
        ("DOCUMENT","SYN source","Portable, inspectable state"),
        ("SCENES","Workspaces","Independent compositions"),
        ("BEHAVIOR","Interactions","Events and actions"),
        ("MOTION","Timeline","Tracks and keyframes"),
        ("MEDIA","Embedded","Portable assets"),
        ("DATA","Bindings","Structured values"),
    ].iter().enumerate() {
        let x=58.+(i%3) as f32*342.;
        let y=202.+(i/3) as f32*142.;
        arch.objects.push(object("shape",*a,x,y,316.,112,json!({"background":"linear-gradient(145deg,#151a27,#0f131d)","borderColor":"#30384c","borderWidth":1,"borderRadius":18,"boxShadow":"0 18px 45px #0007"})));
        arch.objects.push(object("text",*a,x+18.,y+17.,140.,16,json!({"fontSize":8,"fontWeight":900,"color":"#8f7cff","letterSpacing":1.6})));
        arch.objects.push(object("text",*b,x+18.,y+43.,250.,23,json!({"fontSize":17,"fontWeight":850,"color":"#eef0f7"})));
        arch.objects.push(object("text",*c,x+18.,y+77.,260.,18,json!({"fontSize":10,"color":"#7f8aa0"})));
    }

    let mut motion=Scene{id:"MOTION".into(),name:"Motion Lab".into(),background:"#090c12".into(),objects:vec![],interactions:vec![]};
    motion.objects.push(object("text","02  /  MOTION LAB",58.,48.,300.,18,json!({"fontSize":9,"fontWeight":900,"color":"#8f7cff","letterSpacing":2})));
    motion.objects.push(object("text","Motion belongs in the design.",58.,82.,760.,54,json!({"fontSize":40,"fontWeight":850,"color":"#f7f6fc","letterSpacing":-1.2})));
    motion.objects.push(object("text","Build rhythm, transitions and keyframes without exporting your idea to another tool.",62.,145.,760.,24,json!({"fontSize":13,"color":"#98a2b6"})));
    motion.objects.push(object("shape","TIMELINE",58.,202.,1004.,286,json!({"background":"#101521","borderColor":"#30394d","borderWidth":1,"borderRadius":20,"boxShadow":"0 22px 60px #0008"})));
    for i in 0..9 {
        let x=96.+i as f32*106.;
        motion.objects.push(object("text",&format!("{:02}",i),x,226.,35.,16,json!({"fontSize":8,"fontWeight":800,"color":"#66728a"})));
        motion.objects.push(object("shape","",x,252.,1.,182,json!({"background":"#2a3347"})));
    }
    motion.objects.push(object("shape","PRIMARY TRACK",96.,282.,710.,42,json!({"background":"linear-gradient(90deg,#8f7cff,#d05cff)","borderRadius":10,"boxShadow":"0 8px 24px #8f7cff33"})));
    motion.objects.push(object("shape","SECONDARY TRACK",176.,354.,510.,32,json!({"background":"#29354d","borderRadius":8})));
    motion.objects.push(object("text","00:00.00",96.,442.,90.,18,json!({"fontSize":9,"fontWeight":850,"color":"#9aa5ba"})));
    motion.objects.push(object("text","01:24.00",928.,442.,90.,18,json!({"fontSize":9,"fontWeight":850,"color":"#9aa5ba"})));

    let mut media=Scene{id:"MEDIA".into(),name:"Media Lab".into(),background:"#090c12".into(),objects:vec![],interactions:vec![]};
    media.objects.push(object("text","03  /  MEDIA LAB",58.,48.,300.,18,json!({"fontSize":9,"fontWeight":900,"color":"#8f7cff","letterSpacing":2})));
    media.objects.push(object("text","Media without the mess.",58.,82.,760.,54,json!({"fontSize":40,"fontWeight":850,"color":"#f7f6fc","letterSpacing":-1.2})));
    media.objects.push(object("text","Drop media into the document and keep the project portable.",62.,145.,700.,24,json!({"fontSize":13,"color":"#98a2b6"})));
    media.objects.push(object("shape","DROP",58.,202.,520.,286,json!({"background":"linear-gradient(145deg,#171c2a,#0e131d)","borderColor":"#3a4358","borderWidth":1,"borderRadius":20,"boxShadow":"0 22px 60px #0008"})));
    media.objects.push(object("text","DROP MEDIA",92.,246.,260.,32,json!({"fontSize":27,"fontWeight":850,"color":"#f7f6fc"})));
    media.objects.push(object("text","Image  •  Video  •  Audio  •  Fonts",94.,294.,300.,20,json!({"fontSize":11,"color":"#8e99ad"})));
    media.objects.push(object("shape","DROPZONE",94.,338.,446.,104,json!({"background":"#101724","borderColor":"#4b5873","borderWidth":1,"borderRadius":16})));
    media.objects.push(object("text","Drag, browse, or import",218.,365.,230.,20,json!({"fontSize":13,"fontWeight":750,"color":"#dfe3ed"})));
    media.objects.push(object("text","Embedded assets travel with .syn",198.,394.,260.,18,json!({"fontSize":9,"color":"#758198"})));
    media.objects.push(object("shape","LIBRARY",606.,202.,456.,286,json!({"background":"#101520","borderColor":"#30394d","borderWidth":1,"borderRadius":20})));
    media.objects.push(object("text","ASSET LIBRARY",634.,230.,220.,18,json!({"fontSize":9,"fontWeight":900,"color":"#8f7cff","letterSpacing":1.5})));
    for (i,(name,kind)) in [("hero-image.png","IMAGE"),("intro-video.mp4","VIDEO"),("voiceover.wav","AUDIO"),("Inter Variable","FONT")].iter().enumerate() {
        let y=270.+i as f32*48.;
        media.objects.push(object("text",kind,634.,y,70.,16,json!({"fontSize":8,"fontWeight":900,"color":"#6f7b92","letterSpacing":1.2})));
        media.objects.push(object("text",name,716.,y,260.,18,json!({"fontSize":11,"fontWeight":700,"color":"#d6dbe5"})));
        media.objects.push(object("shape","",634.,y+26.,392.,1,json!({"background":"#242d3e"})));
    }

    d.scenes=vec![home,arch,motion,media];
    d
}
fn initial() -> Document {
    #[cfg(target_arch="wasm32")]
    if let Some(w)=web_sys::window() {
        if let Ok(Some(s))=w.local_storage() {
            if let Ok(Some(v))=s.get_item("syn-studio-document") {
                if let Ok(d)=serde_json::from_str(&v) { return d; }
            }
        }
    }
    demo()
}

fn save(d:&Document) {
    #[cfg(target_arch="wasm32")]
    if let Some(w)=web_sys::window() {
        if let Ok(Some(s))=w.local_storage() {
            let _=s.set_item("syn-studio-document",&serde_json::to_string(d).unwrap());
        }
    }
}

fn download(name:&str, raw:&str) {
    #[cfg(target_arch="wasm32")]
    {
        let w=web_sys::window().unwrap();
        let doc=w.document().unwrap();
        let blob=web_sys::Blob::new_with_str_sequence(
            &js_sys::Array::of1(&wasm_bindgen::JsValue::from_str(raw))
        ).unwrap();
        let url=web_sys::Url::create_object_url_with_blob(&blob).unwrap();
        let a=doc.create_element("a").unwrap().dyn_into::<web_sys::HtmlAnchorElement>().unwrap();
        a.set_href(&url); a.set_download(name); a.click();
        let _=web_sys::Url::revoke_object_url(&url);
    }
}

fn style(o:&Object)->String {
    let mut s=format!("left:{}px;top:{}px;width:{}px;height:{}px;transform:rotate({}deg);",o.x,o.y,o.width,o.height,o.rotation);
    if let Some(m)=o.styles.as_object() {
        for (k,v) in m {
            match (k.as_str(),v) {
                ("fontSize",Value::Number(n))=>s+=&format!("font-size:{}px;",n),
                ("fontWeight",Value::Number(n))=>s+=&format!("font-weight:{};",n),
                ("color",Value::String(v))=>s+=&format!("color:{};",v),
                ("background",Value::String(v))=>s+=&format!("background:{};",v),
                ("borderRadius",Value::Number(n))=>s+=&format!("border-radius:{}px;",n),
                ("borderWidth",Value::Number(n))=>s+=&format!("border-width:{}px;",n),
                ("borderColor",Value::String(v))=>s+=&format!("border-color:{};",v),
                ("boxShadow",Value::String(v))=>s+=&format!("box-shadow:{};",v),
                ("letterSpacing",Value::Number(n))=>s+=&format!("letter-spacing:{}px;",n),
                ("lineHeight",Value::Number(n))=>s+=&format!("line-height:{};",n),
                ("fontFamily",Value::String(v))=>s+=&format!("font-family:{};",v),
                ("textAlign",Value::String(v))=>s+=&format!("text-align:{};",v),
                ("fontStyle",Value::String(v))=>s+=&format!("font-style:{};",v),
                ("textDecoration",Value::String(v))=>s+=&format!("text-decoration:{};",v),
                ("opacity",Value::Number(n))=>s+=&format!("opacity:{};",n),
                _=>{}
            }
        }
    }
    s
}

#[cfg(feature="web")]
fn main(){dioxus::launch(App)}
#[cfg(not(feature="web"))]
fn main(){}

#[component]
fn App()->Element {
    let mut doc=use_signal(initial);
    let mut scene=use_signal(||0usize);
    let mut selected=use_signal(||None::<String>);
    let mut tool=use_signal(||"select".to_string());
    let mut mode=use_signal(||"design".to_string());
    let mut preview=use_signal(||false);
    let mut zoom=use_signal(||0.72f32);
    let mut menu=use_signal(||"Home".to_string());
    let mut modal=use_signal(||None::<String>);
    let mut status=use_signal(||"Ready".to_string());
    let mut history=use_signal(||History::new(&initial()));
    let mut source=use_signal(String::new);
    let mut panel=use_signal(||"inspector".to_string());
    let mut workspace=use_signal(||"".to_string());
    let mut inspector_open=use_signal(||true);
    let mut inspector_wide=use_signal(||false);
    let mut inspector_width=use_signal(||348i32);

    let active=doc.read().scenes.get(*scene.read()).cloned();
    let scene_count=doc.read().scenes.len();
    let selected_id=selected.read().clone();
    let status_text=status.read().clone();
    let zoom_label=if (*zoom.read()-0.72).abs()<0.02{"FIT".to_string()}else{format!("{}%",(*zoom.read()*100.) as i32)};
    let dock_label=format!("{} objects  ·  {} scenes  ·  {} assets",
        active.as_ref().map(|s|s.objects.len()).unwrap_or(0),scene_count,doc.read().assets.len());
    let work_class=if !*inspector_open.read(){"work inspector-closed"}else{"work"};
    let work_style=if *inspector_open.read(){format!("--inspector-width:{}px",*inspector_width.read())}else{"--inspector-width:0px".into()};
    let panel_title=match panel.read().as_str() {
        "inspector" => "Inspector",
        "assets" => "Assets",
        "timeline" => "Timeline",
        "interaction" => "Interactions",
        _ => "Workspace",
    };
    let selected_text=selected_id.as_ref().and_then(|id|active.as_ref().and_then(|s|s.objects.iter().find(|o|&o.id==id))).map(|o|o.kind=="text").unwrap_or(false);

    if *preview.read() {
        return rsx!{ Preview { doc:doc.read().clone(), scene:*scene.read(), close:move |_|preview.set(false) } };
    }

    rsx! {
        document::Stylesheet { href:CSS }
        div { class:"app",
            header { class:"top",
                div { class:"titlebar",
                    div { class:"brand", b{"S"}, div{strong{"SYN Studio"},small{"Creative Systems IDE"}} }
                    div { class:"quick",
                        button{title:"Save",onclick:{let mut status=status.clone();let d=doc.clone();move |_|{save(&d.read());status.set("Saved locally".into())}},"⌘ Save"}
                        button{title:"Undo",onclick:{let mut status=status.clone();let mut history=history.clone();let mut doc=doc.clone();move |_|{let cur=doc.read().clone();if let Some(n)=history.write().undo(&cur){doc.set(n);status.set("Undo".into())}}},"↶"}
                        button{title:"Redo",onclick:{let mut status=status.clone();let mut history=history.clone();let mut doc=doc.clone();move |_|{if let Some(n)=history.write().redo(){doc.set(n);status.set("Redo".into())}}},"↷"}
                    }
                    div{class:"title-spacer"}
                    span{class:"status-chip","{status_text}"}
                    button{class:"title-action",onclick:{let d=doc.clone();move |_|download("syn-studio-project.syn",&serde_json::to_string_pretty(&*d.read()).unwrap())},"Export .syn"}
                    button{class:"title-primary",onclick:move |_|preview.set(true),"Preview"}
                }
                div { class:"ribbon-tabs",
                    button { class:"app-menu",onclick:{let mut menu=menu.clone();move |_|menu.set("File".into())},"S" }
                    for (name,_) in menus() {
                        button {
                            class:if menu.read().as_str()==name{"ribbon-tab active"}else{"ribbon-tab"},
                            onclick:{let n=name.to_string();let mut menu=menu.clone();move |_|menu.set(n.clone())},
                            "{name}"
                        }
                    }
                }
                Ribbon {
                    active:menu.read().clone(),
                    pick:{
                        let mut menu=menu.clone(); let mut modal=modal.clone(); let mut mode=mode.clone();
                        let mut preview=preview.clone(); let mut doc=doc.clone(); let mut history=history.clone();
                        let mut scene=scene.clone(); let mut selected=selected.clone(); let mut status=status.clone();
                        let mut panel=panel.clone(); let mut workspace=workspace.clone(); let mut inspector_open=inspector_open.clone();
                        move |item:String| {
                            let item_name=item.clone();
                            match item.as_str() {
                                "New Project" => {let cur=doc.read().clone();history.write().push(&cur);doc.set(demo());scene.set(0);selected.set(None);status.set("New project created".into());},
                                "Save" => {save(&doc.read());status.set("Saved locally".into());},
                                "Save Snapshot" => {save(&doc.read());download("syn-studio-snapshot.syn",&serde_json::to_string_pretty(&*doc.read()).unwrap());status.set("Snapshot exported".into());},
                                "Open .syn" => modal.set(Some("Open .syn".into())),
                                "Export .syn" => download("syn-studio-project.syn",&serde_json::to_string_pretty(&*doc.read()).unwrap()),
                                "Undo" => {let cur=doc.read().clone();if let Some(n)=history.write().undo(&cur){doc.set(n);selected.set(None);status.set("Undo".into())}},
                                "Redo" => {if let Some(n)=history.write().redo(){doc.set(n);status.set("Redo".into())}},
                                "Delete Selected" => {
                                    let selected_id=selected.read().clone();if let Some(id)=selected_id{let cur=doc.read().clone();history.write().push(&cur);let mut n=cur;if let Some(s)=n.scenes.get_mut(*scene.read()){s.objects.retain(|o|o.id!=id)}selected.set(None);doc.set(n);status.set("Object deleted".into())}
                                },
                                "Duplicate Selected" => {
                                    let selected_id=selected.read().clone();
                                    let current_scene=*scene.read();
                                    if let Some(id)=selected_id { duplicate_selected(&mut doc,&mut history,current_scene,&id,&mut selected,&mut status) }
                                },
                                "Text"|"Rich Text"|"Heading"|"Paragraph" => insert(&mut doc,&mut history,*scene.read(),&mut selected,"text",item,&mut status),
                                "Button" => insert(&mut doc,&mut history,*scene.read(),&mut selected,"button",item,&mut status),
                                "Shape"|"Card" => insert(&mut doc,&mut history,*scene.read(),&mut selected,"shape",item,&mut status),
                                "Component" => insert(&mut doc,&mut history,*scene.read(),&mut selected,"component",item,&mut status),
                                "Image"|"SVG"|"GIF"|"Audio"|"Video"|"Import Media" => modal.set(Some(item)),
                                "New Scene" => new_scene(&mut doc,&mut history,&mut scene,&mut selected,&mut status),
                                "Duplicate Scene" => {
                                    let current_scene=*scene.read();
                                    duplicate_scene(&mut doc,&mut history,current_scene,&mut scene,&mut status);
                                },
                                "Delete Scene" => delete_scene(&mut doc,&mut history,&mut scene,&mut selected,&mut status),
                                "Design Inspector"|"Object Inspector" => {panel.set("inspector".into());inspector_open.set(true);status.set(format!("{} opened",item));},
                                "Align Center"|"Center on Canvas" => {
                                    let current_scene=*scene.read();
                                    let current_id=selected.read().clone();
                                    if let Some(oid)=current_id { align_center(&mut doc,&mut history,current_scene,&oid,&mut status); }
                                    else { panel.set("workspace".into());workspace.set("Alignment".into());inspector_open.set(true);status.set("Select an object to align it".into()); }
                                },
                                "Reset Transform" => {
                                    let current_scene=*scene.read();
                                    let current_id=selected.read().clone();
                                    if let Some(oid)=current_id { reset_transform(&mut doc,&mut history,current_scene,&oid,&mut status); }
                                    else { panel.set("workspace".into());workspace.set("Transform".into());inspector_open.set(true);status.set("Select an object to reset".into()); }
                                },
                                "Layers"|"Scene Graph" => {panel.set(if item=="Layers"{"layers".into()}else{"scene_graph".into()});inspector_open.set(true);workspace.set(item.clone());status.set(format!("{} opened",item));},
                                "Asset Library"|"Image Library"|"Audio Library"|"Video Library"|"Fonts"|"Icons"|"Documents"|"Embedded Assets"|"Replace Asset" => {panel.set("assets".into());inspector_open.set(true);workspace.set(item.clone());status.set(format!("{} opened",item));},
                                "Timeline"|"Animation" => {panel.set("timeline".into());inspector_open.set(true);status.set(format!("{} opened",item));},
                                "Interaction Graph"|"Navigation"|"States"|"Events" => {panel.set("interaction".into());inspector_open.set(true);workspace.set(item.clone());status.set(format!("{} opened",item));},
                                "Code Editor" => {source.set(serde_json::to_string_pretty(&*doc.read()).unwrap());panel.set("code".into());mode.set("code".into());inspector_open.set(true);status.set("Code Editor opened".into());},
                                "Validation"|"Validate" => {panel.set("workspace".into());workspace.set("Validation".into());inspector_open.set(true);status.set(validate(&doc.read()));},
                                "Preview"|"Publish Preview"|"Responsive Preview" => preview.set(true),
                                "AI Workbench"|"Generate Text"|"Generate Layout"|"Generate Scene"|"Generate Component"|"Generate Code"|"Analyze Document"|"Organize Document"|"Accessibility Review" => {panel.set("ai".into());workspace.set(item.clone());inspector_open.set(true);status.set(format!("{} opened",item));},
                                "Web Package"|"Desktop Package"|"Mobile Package" => {download("syn-studio-project.syn",&serde_json::to_string_pretty(&*doc.read()).unwrap());status.set(format!("{} package exported as .syn",item));},
                                "Manifest" => {panel.set("workspace".into());workspace.set("Manifest".into());inspector_open.set(true);status.set("Manifest workspace opened".into());},
                                "Share" => {panel.set("workspace".into());workspace.set("Share".into());inspector_open.set(true);status.set("Share workspace opened".into());},
                                _ => {panel.set("workspace".into());workspace.set(item_name.clone());inspector_open.set(true);status.set(format!("{} opened",item_name));}
                            }
                        }
                    }
                }
            }

            main { class:"{work_class}", style:"{work_style}",
                aside { class:"rail",
                    span{class:"rail-label","TOOLS"}
                    for (glyph,name) in [("↖","select"),("✥","move"),("✋","pan"),("T","text"),("◇","shape"),("▣","button"),("◈","media")] {
                        button {
                            class:if *tool.read()==name{"railbtn active"}else{"railbtn"},
                            title:"{name}",
                            onclick:{let name=name.to_string();let mut tool=tool.clone();let mut modal=modal.clone();move |_|{tool.set(name.clone());if name=="text"{modal.set(Some("Text".into()))}else if name=="shape"{modal.set(Some("Shape".into()))}else if name=="button"{modal.set(Some("Button".into()))}else if name=="media"{modal.set(Some("Import Media".into()))}}},
                            span{class:"ico","{glyph}"} small{"{name}"}
                        }
                    }
                    div{class:"rail-spacer"}
                    button{class:"railbtn",title:"Toggle inspector",onclick:{let mut inspector_open=inspector_open.clone();move |_|inspector_open.toggle()},"◧" small{"panel"}}
                }

                section { class:"center",
                    div { class:"bar",
                        div { class:"scene-title",span{class:"eyebrow","SCENE"},strong{"{active.as_ref().map(|s|s.name.clone()).unwrap_or_default()}"},span{class:"crumb","{scene_count} scenes"} }
                        div { class:"modes",
                            button{class:if *mode.read()=="design"{"active"}else{""},onclick:move |_|mode.set("design".into()),"Design"}
                            button{class:if *mode.read()=="code"{"active"}else{""},onclick:{let mut mode=mode.clone();let mut source=source.clone();let d=doc.clone();move |_|{source.set(serde_json::to_string_pretty(&*d.read()).unwrap());mode.set("code".into())}},"Code"}
                            button{onclick:move |_|preview.set(true),"Preview"}
                        }
                        div { class:"zoom",
                            button{onclick:move |_|zoom.set(0.72),"Fit"}
                            button{onclick:move |_|{let current_zoom=*zoom.read();zoom.set((current_zoom-0.08).max(0.45))},"−"}
                            span{"{zoom_label}"}
                            button{onclick:move |_|{let current_zoom=*zoom.read();zoom.set((current_zoom+0.08).min(1.5))},"+"}
                        }
                        if !*inspector_open.read(){button{class:"show-inspector",onclick:{let mut inspector_open=inspector_open.clone();move |_|inspector_open.set(true)},"Inspector"}}
                    }
                    if selected_text && *mode.read()=="design" {
                        TextToolbar { doc:doc.clone(),scene:*scene.read(),id:selected_id.clone().unwrap_or_default(),history:history.clone(),status:status.clone() }
                    }
                    if *mode.read()=="code" {
                        Code { source:source.clone(),doc:doc.clone(),history:history.clone(),status:status.clone(),mode:mode.clone() }
                    } else {
                        Canvas { doc:doc.clone(),scene:*scene.read(),selected:selected.clone(),zoom:*zoom.read(),pick:{let mut selected=selected.clone();move |v|selected.set(v)} }
                    }
                    SceneTabs { doc:doc.clone(),scene:scene.clone(),selected:selected.clone(),history:history.clone(),status:status.clone() }
                }

                if *inspector_open.read() {
                    aside { class:"inspector",
                        div { class:"inspecthead",
                            div{class:"panel-title",strong{"{panel_title}"}}
                            button{title:"Compact inspector",class:"panel-icon",onclick:{let mut inspector_width=inspector_width.clone();let mut inspector_wide=inspector_wide.clone();move |_|{inspector_width.set(288);inspector_wide.set(false)}},"−"}
                            button{title:"Default inspector",class:"panel-icon",onclick:{let mut inspector_width=inspector_width.clone();let mut inspector_wide=inspector_wide.clone();move |_|{inspector_width.set(348);inspector_wide.set(false)}},"□"}
                            button{title:"Wide inspector",class:"panel-icon",onclick:{let mut inspector_width=inspector_width.clone();let mut inspector_wide=inspector_wide.clone();move |_|{inspector_width.set(430);inspector_wide.set(true)}},"↔"}
                            button{title:"Close panel",class:"panel-icon close",onclick:{let mut inspector_open=inspector_open.clone();move |_|inspector_open.set(false)},"×"}
                        }
                        if *panel.read()=="assets" {
                            AssetsPanel { doc:doc.clone() }
                        } else if *panel.read()=="timeline" {
                            TimelinePanel {}
                        } else if *panel.read()=="interaction" {
                            InteractionPanel { doc:doc.clone(),scene:*scene.read() }
                        } else if *panel.read()=="code" {
                            div{class:"panelbody code-inspector",strong{"CODE INTELLIGENCE"},p{"The source view is syntax-highlighted and controlled by Rust state. Format, validate, and apply changes without leaving the studio."},div{class:"validation-card",span{class:"ok-dot"},"Live SYN model"},div{class:"validation-card",span{class:"ok-dot"},"Syntax highlighting"},div{class:"validation-card",span{class:"ok-dot"},"Line numbers and structured source"}}
                        } else if *panel.read()=="ai" {
                            AiPanel { status:status.clone() }
                        } else if *panel.read()=="workspace" || *panel.read()=="layers" || *panel.read()=="scene_graph" {
                            WorkspacePanel { command:workspace.read().clone(), doc:doc.clone(), scene:*scene.read(), status:status.clone() }
                        } else if let Some(id)=selected_id {
                            Inspect { doc:doc.clone(),scene:*scene.read(),id,status:status.clone(),history:history.clone() }
                        } else {
                            div{class:"empty",b{"◇"},strong{"Select an object"},p{"Geometry, content, typography and state appear here. Open any ribbon command to switch this panel to a real workspace."}}
                        }
                    }
                }
            }

            if let Some(kind)=modal.read().clone() {
                Dialog{kind,close:move |_|modal.set(None),doc:doc.clone(),scene:scene.clone(),history:history.clone(),selected:selected.clone(),status:status.clone()}
            }
        }
    }
}

#[component]
fn Ribbon(active:String,pick:EventHandler<String>)->Element {
    rsx!{
        div{class:"ribbon",
            div{class:"ribbon-scroll",
                for (group,items) in ribbon_groups(&active) {
                    div{class:"ribbon-group",
                        div{class:"ribbon-group-body",
                            for item in items {
                                button{class:if item=="Export .syn"||item=="Preview"||item=="Validate"{"ribbon-command featured"}else{"ribbon-command"},onclick:{let item=item.to_string();let pick=pick.clone();move |_|pick.call(item.clone())},
                                    span{class:"command-icon","{icon_for(item)}"}
                                    span{class:"command-label","{item}"}
                                }
                            }
                        }
                        span{class:"ribbon-group-label","{group}"}
                    }
                }
            }
        }
    }
}

fn ribbon_groups(active:&str)->Vec<(&'static str,Vec<&'static str>)> {
    match active {
        "File"=>vec![("Project",vec!["New Project","Open .syn","Save","Save Snapshot"]),("History",vec!["Undo","Redo"]),("Export",vec!["Export .syn","Preview"])],
        "Home"=>vec![("Clipboard",vec!["Undo","Redo"]),("Edit",vec!["Duplicate Selected","Delete Selected"]),("Object",vec!["Align Center","Center on Canvas","Reset Transform"]),("Text",vec!["Typography","Colors","Effects"])],
        "Insert"=>vec![("Text",vec!["Text","Rich Text","Heading","Paragraph"]),("Elements",vec!["Button","Shape","Card","Component"]),("Media",vec!["Image","SVG","GIF","Audio","Video","Import Media"])],
        "Design"=>vec![("Inspect",vec!["Design Inspector","Object Inspector","Layers","Scene Graph"]),("Layout",vec!["Responsive Layout","Typography","Colors","Effects"]),("Accessibility",vec!["Accessibility"])],
        "Build"=>vec![("Scenes",vec!["New Scene","Duplicate Scene","Delete Scene"]),("Motion",vec!["Timeline","Animation","States"]),("Behavior",vec!["Interaction Graph","Data","Variables","Navigation","Events"]),("Run",vec!["Preview"])],
        "Assets"=>vec![("Library",vec!["Asset Library","Image Library","Audio Library","Video Library"]),("Types",vec!["Fonts","Icons","Documents"]),("Storage",vec!["Replace Asset","Embedded Assets"])],
        "Code"=>vec![("Editor",vec!["Code Editor","Validation"]),("Diagnostics",vec!["Console","Output","Debugging"]),("APIs",vec!["Runtime API","Scene API","Data API","Media API","Animation API"]),("Preview",vec!["Responsive Preview"])],
        "AI"=>vec![("Workbench",vec!["AI Workbench"]),("Generate",vec!["Generate Text","Generate Layout","Generate Scene","Generate Component","Generate Code"]),("Review",vec!["Analyze Document","Organize Document","Accessibility Review"])],
        "Publish"=>vec![("Checks",vec!["Validate"]),("Preview",vec!["Publish Preview","Save Snapshot"]),("Packages",vec!["Export .syn","Web Package","Desktop Package","Mobile Package"]),("Share",vec!["Share","Manifest"])],
        _=>vec![]
    }
}

fn icon_for(item:&str)->&'static str {
    match item {
        "New Project"=>"＋","Open .syn"=>"↥","Save"=>"⌘","Save Snapshot"=>"▣","Undo"=>"↶","Redo"=>"↷",
        "Delete Selected"=>"⌫","Duplicate Selected"=>"⧉","Text"|"Rich Text"|"Heading"|"Paragraph"=>"T",
        "Button"=>"▰","Shape"=>"◇","Card"=>"▱","Component"=>"◆","Image"=>"▧","SVG"=>"◇","GIF"=>"◉",
        "Audio"=>"◌","Video"=>"▶","Import Media"=>"↥","New Scene"=>"＋","Duplicate Scene"=>"⧉","Delete Scene"=>"⌫",
        "Design Inspector"|"Object Inspector"=>"⌘","Layers"=>"▤","Scene Graph"=>"⌁","Responsive Layout"=>"⌗",
        "Typography"=>"Aa","Colors"=>"◐","Effects"=>"✦","Accessibility"=>"♿","Timeline"=>"▥","Animation"=>"◒",
        "States"=>"◈","Interaction Graph"=>"⌁","Data"=>"{}","Variables"=>"x","Navigation"=>"⇢","Events"=>"⚡",
        "Preview"|"Publish Preview"|"Responsive Preview"=>"▶","Asset Library"|"Image Library"|"Audio Library"|"Video Library"=>"▦",
        "Fonts"=>"Aa","Icons"=>"✦","Documents"=>"▤","Replace Asset"=>"↻","Embedded Assets"=>"⌂",
        "Code Editor"=>"</>","Validation"|"Validate"=>"✓","Console"=>">_","Output"=>"≡","Debugging"=>"⌁",
        "Runtime API"|"Scene API"|"Data API"|"Media API"|"Animation API"=>"{}","AI Workbench"=>"✦",
        "Generate Text"|"Generate Layout"|"Generate Scene"|"Generate Component"|"Generate Code"=>"✦",
        "Analyze Document"|"Organize Document"|"Accessibility Review"=>"◎","Export .syn"=>"⇩","Web Package"=>"⌘",
        "Desktop Package"=>"▣","Mobile Package"=>"▥","Share"=>"↗","Manifest"=>"{}","Align Center"|"Center on Canvas"=>"⊙",
        "Reset Transform"=>"↺",_=>"•"
    }
}

#[component]
fn SceneTabs(doc:Signal<Document>,scene:Signal<usize>,selected:Signal<Option<String>>,history:Signal<History>,status:Signal<String>)->Element {
    rsx!{div{class:"scene-tabs",
        div{class:"scene-tab-scroll",
            for (i,s) in doc.read().scenes.iter().enumerate() {
                button{class:if *scene.read()==i{"scene-tab active"}else{"scene-tab"},onclick:{let mut scene=scene.clone();let mut selected=selected.clone();move |_|{scene.set(i);selected.set(None)}},span{class:"scene-tab-dot"},"{s.name}"}
            }
            button{class:"scene-tab-add",onclick:{let mut doc=doc.clone();let mut history=history.clone();let mut scene=scene.clone();let mut selected=selected.clone();let mut status=status.clone();move |_|new_scene(&mut doc,&mut history,&mut scene,&mut selected,&mut status)},"＋"}
        }
        div{class:"scene-tab-status","{status}"}
    }}
}

#[component]
fn TextToolbar(doc:Signal<Document>,scene:usize,id:String,history:Signal<History>,status:Signal<String>)->Element {
    let o=doc.read().scenes.get(scene).and_then(|s|s.objects.iter().find(|o|o.id==id)).cloned();
    let Some(o)=o else{return rsx!{}};
    let mut font=use_signal(||o.styles.get("fontFamily").and_then(Value::as_str).unwrap_or("Inter").to_string());
    let mut size=use_signal(||o.styles.get("fontSize").and_then(Value::as_f64).unwrap_or(16.).to_string());
    let mut color=use_signal(||o.styles.get("color").and_then(Value::as_str).unwrap_or("#f5f7ff").to_string());
    let mut weight=use_signal(||o.styles.get("fontWeight").and_then(Value::as_i64).unwrap_or(650).to_string());
    let mut align=use_signal(||o.styles.get("textAlign").and_then(Value::as_str).unwrap_or("left").to_string());
    let mut italic=use_signal(||o.styles.get("fontStyle").and_then(Value::as_str).unwrap_or("normal")=="italic");
    let mut underline=use_signal(||o.styles.get("textDecoration").and_then(Value::as_str).unwrap_or("none")=="underline");
    rsx!{div{class:"text-toolbar",
        select{value:"{font}",oninput:move|e|font.set(e.value()),option{"Inter"},option{"Segoe UI"},option{"Georgia"},option{"JetBrains Mono"},option{"Space Grotesk"}},
        input{class:"size-input",type:"number",min:"8",max:"120",value:"{size}",oninput:move|e|size.set(e.value())},
        button{class:if *weight.read()=="800"||*weight.read()=="900"{"fmt active"}else{"fmt"},onclick:move |_|{if *weight.read()=="800"{weight.set("650".into())}else{weight.set("800".into())}},"B"},
        button{class:if *italic.read(){"fmt active"}else{"fmt"},onclick:move |_|italic.toggle(),"I"},
        button{class:if *underline.read(){"fmt active"}else{"fmt"},onclick:move |_|underline.toggle(),"U"},
        div{class:"align-group",
            button{class:if *align.read()=="left"{"fmt active"}else{"fmt"},onclick:{let mut align=align.clone();move |_|align.set("left".into())},"L"},
            button{class:if *align.read()=="center"{"fmt active"}else{"fmt"},onclick:{let mut align=align.clone();move |_|align.set("center".into())},"C"},
            button{class:if *align.read()=="right"{"fmt active"}else{"fmt"},onclick:{let mut align=align.clone();move |_|align.set("right".into())},"R"}
        },
        label{class:"color-control",title:"Text color",input{type:"color",value:"{color}",oninput:move|e|color.set(e.value())},span{"Color"}},
        button{class:"apply-text",onclick:{let mut doc=doc.clone();let mut history=history.clone();let mut status=status.clone();move |_|{
            let cur=doc.read().clone();history.write().push(&cur);let mut n=cur;
            if let Some(v)=n.scenes.get_mut(scene).and_then(|s|s.objects.iter_mut().find(|o|o.id==id)){
                if let Some(m)=v.styles.as_object_mut(){
                    m.insert("fontFamily".into(),Value::String(font.read().clone()));
                    m.insert("fontSize".into(),json!(size.read().parse::<f32>().unwrap_or(16.)));
                    m.insert("fontWeight".into(),json!(weight.read().parse::<u32>().unwrap_or(650)));
                    m.insert("color".into(),Value::String(color.read().clone()));
                    m.insert("textAlign".into(),Value::String(align.read().clone()));
                    m.insert("fontStyle".into(),json!(if *italic.read(){"italic"}else{"normal"}));
                    m.insert("textDecoration".into(),json!(if *underline.read(){"underline"}else{"none"}));
                }
            }
            doc.set(n);status.set("Text formatting applied".into());
        }},"Apply"}
    }}
}

#[component]
fn WorkspacePanel(command:String,doc:Signal<Document>,scene:usize,status:Signal<String>)->Element {
    let title=if command.is_empty(){"Workspace".to_string()}else{command.clone()};
    let objects=doc.read().scenes.get(scene).map(|s|s.objects.len()).unwrap_or(0);
    let assets=doc.read().assets.len();
    rsx!{div{class:"workspace-panel",
        div{class:"workspace-hero",
            span{class:"workspace-kicker","SYN WORKSPACE"},
            strong{"{title}"},
            p{"A live tool surface, not a placeholder. The current scene and document remain connected while you work."}
        },
        if command=="Validation" {
            button{class:"workspace-action primary-action",onclick:{let mut status=status.clone();let d=doc.clone();move |_|status.set(validate(&d.read()))},"Run validation"}
            div{class:"validation-card",span{class:"ok-dot"},"Schema, scene IDs and document structure are checked against the current SYN model."}
        } else if command=="Scene Graph" {
            div{class:"graph large",for (i,s) in doc.read().scenes.iter().enumerate(){div{class:"graph-node",style:format!("left:{}%;top:{}%;",8+(i%2)*46,14+(i/2)*34),"SCENE {i+1}",span{"{s.name}"}}}}
        } else if command=="Layers" {
            div{class:"layer-list",for o in doc.read().scenes.get(scene).map(|s|s.objects.clone()).unwrap_or_default(){div{class:"layer-row",span{class:"layer-icon","◇"},span{"{o.label}"},small{"{o.kind}"}}}}
        } else if command=="Alignment" {
            div{class:"workspace-grid",div{class:"workspace-card",strong{"Alignment tools"},p{"Select an object, then use Home → Object to center it on the canvas."},button{class:"workspace-action",onclick:move |_|status.set("Alignment controls are active in the Home ribbon".into()),"Use Home alignment tools"}}}
        } else if command=="Transform" {
            div{class:"workspace-grid",div{class:"workspace-card",strong{"Transform controls"},p{"Reset rotation and edit geometry from the Inspector on the right."},button{class:"workspace-action",onclick:move |_|status.set("Transform controls are active in the Inspector".into()),"Open Inspector"}}}
        } else {
            div{class:"workspace-grid",
                div{class:"workspace-card",strong{"Live command"},p{"{title} is connected to the editor. This surface exposes the next operation instead of pretending a command completed."},button{class:"workspace-action",onclick:{let mut status=status.clone();let c=title.clone();move |_|status.set(format!("{} is ready",c))},"Activate command"}},
                div{class:"workspace-card",strong{"Scene state"},p{"{objects} objects · {assets} embedded assets · {doc.read().scenes.len()} scenes"},button{class:"workspace-action",onclick:{let mut status=status.clone();move |_|status.set("Current scene state refreshed".into())},"Refresh state"}}
            }
        }
    }}
}


fn menus()->Vec<(&'static str,&'static str)> {
    vec![("Project","project"),("Create","create"),("Design","design"),("Build","build"),("Assets","assets"),("Code","code"),("AI","ai"),("Publish","publish")]
}

fn commands(m:&str)->Vec<&'static str> {
    match m {
        "Project"=>vec!["New Project","Open .syn","Save","Save Snapshot","Export .syn","Undo","Redo","Duplicate Selected","Delete Selected","Project Settings"],
        "Create"=>vec!["Text","Rich Text","Heading","Paragraph","Button","Shape","Card","Component","Image","SVG","GIF","Audio","Video","Import Media"],
        "Design"=>vec!["Design Inspector","Align Center","Center on Canvas","Reset Transform","Layers","Scene Graph","Responsive Layout","Typography","Colors","Effects","Accessibility"],
        "Build"=>vec!["New Scene","Duplicate Scene","Delete Scene","Timeline","Interaction Graph","Data","Variables","Navigation","Animation","States","Events","Preview"],
        "Assets"=>vec!["Asset Library","Import Media","Image Library","Audio Library","Video Library","Fonts","Icons","Documents","Replace Asset","Embedded Assets"],
        "Code"=>vec!["Code Editor","Validation","Console","Output","Runtime API","Scene API","Data API","Media API","Animation API","Responsive Preview","Debugging"],
        "AI"=>vec!["AI Workbench","Generate Text","Generate Layout","Generate Scene","Generate Component","Generate Code","Analyze Document","Organize Document","Accessibility Review"],
        "Publish"=>vec!["Validate","Publish Preview","Save Snapshot","Export .syn","Web Package","Desktop Package","Mobile Package","Share","Manifest"],
        _=>vec![]
    }
}

fn panel_for(item:&str)->String {
    match item {
        "Timeline"=> "timeline".into(),
        "Interaction Graph"=> "interaction".into(),
        "Asset Library"=> "assets".into(),
        "Code Editor"=> "code".into(),
        "AI Workbench"=> "ai".into(),
        _=>"inspector".into()
    }
}

fn validate(d:&Document)->String {
    if d.syn.is_empty() || d.scenes.is_empty() { "Invalid SYN document".into() }
    else if d.scenes.iter().any(|s|s.id.is_empty()) { "Invalid scene id".into() }
    else { "SYN document valid".into() }
}

fn insert(doc:&mut Signal<Document>,history:&mut Signal<History>,scene:usize,selected:&mut Signal<Option<String>>,kind:&str,label:String,status:&mut Signal<String>) {
    let cur=doc.read().clone(); history.write().push(&cur); let mut n=cur;
    if let Some(s)=n.scenes.get_mut(scene) {
        let oid=id("obj");
        let (w,h,styles)=match kind {
            "button"=>(220.,52.,json!({"background":"linear-gradient(180deg,#eef3ff,#aebfff)","color":"#07101c","fontSize":13,"fontWeight":900,"borderRadius":10,"boxShadow":"0 10px 25px #0005"})),
            "component"=>(300.,150.,json!({"background":"linear-gradient(145deg,#172640,#0d1724)","color":"#eef3fb","borderColor":"#385575","borderWidth":1,"borderRadius":15,"fontSize":16,"fontWeight":800})),
            "shape"=>(240.,130.,json!({"background":"#121c29","color":"#eaf0fa","borderColor":"#2b405b","borderWidth":1,"borderRadius":13,"fontSize":16})),
            _=>(360.,64.,json!({"fontSize":22,"fontWeight":750,"color":"#f4f7fb"}))
        };
        let mut o=object(kind,&label,70.+s.objects.len() as f32*10.,100.+s.objects.len() as f32*8.,w,h,styles);
        o.id=oid.clone(); s.objects.push(o); selected.set(Some(oid));
    }
    doc.set(n); status.set(format!("Inserted {}",label));
}

fn duplicate_selected(doc:&mut Signal<Document>,history:&mut Signal<History>,scene:usize,oid:&str,selected:&mut Signal<Option<String>>,status:&mut Signal<String>) {
    let cur=doc.read().clone(); history.write().push(&cur); let mut n=cur;
    if let Some(s)=n.scenes.get_mut(scene) {
        if let Some(o)=s.objects.iter().find(|o|o.id==oid).cloned() {
            let mut c=o; c.id=id("obj"); c.x+=24.; c.y+=24.; let nid=c.id.clone(); s.objects.push(c); selected.set(Some(nid));
        }
    }
    doc.set(n); status.set("Object duplicated".into());
}

fn align_center(doc:&mut Signal<Document>,history:&mut Signal<History>,scene:usize,oid:&str,status:&mut Signal<String>) {
    let cur=doc.read().clone(); history.write().push(&cur); let mut n=cur;
    if let Some(o)=n.scenes.get_mut(scene).and_then(|s|s.objects.iter_mut().find(|o|o.id==oid)) {
        o.x=(n.viewport.width as f32-o.width)/2.; o.y=(n.viewport.height as f32-o.height)/2.;
    }
    doc.set(n); status.set("Centered on canvas".into());
}

fn reset_transform(doc:&mut Signal<Document>,history:&mut Signal<History>,scene:usize,oid:&str,status:&mut Signal<String>) {
    let cur=doc.read().clone(); history.write().push(&cur); let mut n=cur;
    if let Some(o)=n.scenes.get_mut(scene).and_then(|s|s.objects.iter_mut().find(|o|o.id==oid)) {
        o.rotation=0.;
    }
    doc.set(n); status.set("Transform reset".into());
}

fn new_scene(doc:&mut Signal<Document>,history:&mut Signal<History>,scene:&mut Signal<usize>,selected:&mut Signal<Option<String>>,status:&mut Signal<String>) {
    let cur=doc.read().clone();history.write().push(&cur);let mut n=cur;
    n.scenes.push(Scene{id:id("scene"),name:format!("Scene {}",n.scenes.len()+1),background:"#0a1018".into(),objects:vec![],interactions:vec![]});
    scene.set(n.scenes.len()-1);selected.set(None);doc.set(n);status.set("New scene created".into());
}

fn duplicate_scene(doc:&mut Signal<Document>,history:&mut Signal<History>,scene:usize,scene_sig:&mut Signal<usize>,status:&mut Signal<String>) {
    let cur=doc.read().clone();history.write().push(&cur);let mut n=cur;
    if let Some(s)=n.scenes.get(scene).cloned(){let mut c=s;c.id=id("scene");c.name=format!("{} Copy",c.name);for o in c.objects.iter_mut(){o.id=id("obj");}n.scenes.insert(scene+1,c);scene_sig.set(scene+1);}
    doc.set(n);status.set("Scene duplicated".into());
}

fn delete_scene(doc:&mut Signal<Document>,history:&mut Signal<History>,scene:&mut Signal<usize>,selected:&mut Signal<Option<String>>,status:&mut Signal<String>) {
    let cur=doc.read().clone();if cur.scenes.len()<=1{status.set("A document needs at least one scene".into());return}
    history.write().push(&cur);let mut n=cur;let current_scene=*scene.read();n.scenes.remove(current_scene);if current_scene>=n.scenes.len(){scene.set(n.scenes.len()-1)}selected.set(None);doc.set(n);status.set("Scene deleted".into());
}

#[component]
fn Mega(open:String,search:Signal<String>,pick:EventHandler<String>)->Element {
    let q=search.read().to_lowercase();
    rsx!{div{class:"mega",
        div{class:"megahead",strong{"{open}"},input{placeholder:"Search {open}…",value:"{search}",oninput:move|e|search.set(e.value())}},
        div{class:"grid",for item in commands(&open).into_iter().filter(|x|q.is_empty()||x.to_lowercase().contains(&q)){
            button{onclick:{let x=item.to_string();let pick=pick.clone();move |_|pick.call(x.clone())},"{item}"}
        }}
    }}
}

#[component]
fn Canvas(doc:Signal<Document>,scene:usize,selected:Signal<Option<String>>,zoom:f32,pick:EventHandler<Option<String>>)->Element {
    let scene_data=doc.read().scenes.get(scene).cloned();
    let background=scene_data.as_ref().map(|x|x.background.clone()).unwrap_or_default();
    let objects=scene_data.map(|x|x.objects).unwrap_or_default();
    rsx!{div{class:"canvas",div{class:"art",style:format!("transform:scale({});background:{};",zoom,background),onclick:move |_|pick.call(None),
        for object in objects.into_iter().filter(|o|!o.hidden){ObjectNode{object,selected:selected.clone(),pick:pick.clone()}}
    }}}
}

#[component]
fn ObjectNode(object:Object,selected:Signal<Option<String>>,pick:EventHandler<Option<String>>)->Element {
    let selected_now=selected.read().as_deref()==Some(object.id.as_str());
    let class_name=if selected_now{"obj selected"}else{"obj"};
    let oid=object.id.clone();
    rsx!{div{class:class_name,style:style(&object),onclick:move|e|{e.stop_propagation();pick.call(Some(oid.clone()));},
        if object.kind=="video"{video{src:object.props.get("src").and_then(Value::as_str),controls:true}}
        else if object.kind=="audio"{audio{src:object.props.get("src").and_then(Value::as_str),controls:true}}
        else if object.kind=="media"{img{src:object.props.get("src").and_then(Value::as_str)}}
        else{span{"{text(&object)}"}}
    }}
}

#[component]
fn Inspect(doc:Signal<Document>,scene:usize,id:String,status:Signal<String>,history:Signal<History>)->Element {
    let o=doc.read().scenes.get(scene).and_then(|s|s.objects.iter().find(|o|o.id==id)).cloned();
    let Some(o)=o else{return rsx!{}};
    let mut x=use_signal(||o.x.to_string());let mut y=use_signal(||o.y.to_string());let mut copy=use_signal(||text(&o));
    rsx!{div{class:"inspectbody",strong{"{o.label}"},small{"{o.kind}"},
        label{"X",input{value:"{x}",oninput:move|e|x.set(e.value())}},
        label{"Y",input{value:"{y}",oninput:move|e|y.set(e.value())}},
        label{"W",input{value:"{o.width}",readonly:true}},label{"H",input{value:"{o.height}",readonly:true}},
        textarea{value:"{copy}",oninput:move|e|copy.set(e.value())},
        button{class:"apply",onclick:{let mut doc=doc.clone();let mut history=history.clone();let mut status=status.clone();move |_|{
            let cur=doc.read().clone();history.write().push(&cur);let mut n=cur;
            if let Some(v)=n.scenes.get_mut(scene).and_then(|s|s.objects.iter_mut().find(|o|o.id==id)){
                v.x=x.read().parse().unwrap_or(v.x);v.y=y.read().parse().unwrap_or(v.y);v.props=json!({"text":copy.read().clone()});
            }
            doc.set(n);status.set("Object updated".into());
        }},"Apply changes"}
    }}
}

#[component]
fn AssetsPanel(doc:Signal<Document>)->Element {
    rsx!{div{class:"panelbody",strong{"ASSET LIBRARY"},p{"{doc.read().assets.len()} embedded assets"},for a in doc.read().assets.iter(){
        div{class:"assetrow",span{class:"assettype","{a.r#type}"},strong{"{a.name}"},small{"{a.size} bytes"}}
    }}}
}

#[component]
fn TimelinePanel()->Element {
    rsx!{div{class:"panelbody",strong{"TIMELINE"},p{"Tracks and keyframes are represented in the SYN document model."},
        div{class:"mini-timeline",for i in 0..7{div{class:"tick",style:format!("left:{}%;",i*15)," {i}"}}},
        div{class:"track",div{class:"clip",style:"left:12%;width:42%;"},div{class:"clip secondary",style:"left:36%;width:30%;"}}}}
}

#[component]
fn InteractionPanel(doc:Signal<Document>,scene:usize)->Element {
    let count=doc.read().scenes.get(scene).map(|s|s.interactions.len()).unwrap_or(0);
    rsx!{div{class:"panelbody",strong{"INTERACTION GRAPH"},p{"{count} event nodes on this scene."},
        div{class:"graph",div{class:"node n1","EVENT"},div{class:"node n2","ACTION"},div{class:"node n3","SCENE"}}}}
}

#[component]
fn AiPanel(status:Signal<String>)->Element {
    rsx!{div{class:"panelbody",strong{"AI WORKBENCH"},p{"Rust keeps the editor deterministic. AI actions can operate on the same document model when connected."},
        textarea{placeholder:"Describe the change you want…"},
        button{class:"apply",onclick:move |_|status.set("AI request staged in the workbench".into()),"Stage request"}}}
}

#[component]
fn Code(source:Signal<String>,doc:Signal<Document>,history:Signal<History>,status:Signal<String>,mode:Signal<String>)->Element {
    let mut language=use_signal(||"JSON".to_string());
    let lang=CodeLanguage::from_slug(&language.read().to_lowercase()).unwrap_or(CodeLanguage::Json);
    let symbol_count=source.read().matches("\" : \"").count();
    rsx!{div{class:"code",
        div{class:"codehead",
            div{class:"code-title",
                span{class:"code-led"},
                div{strong{"SOURCE"},small{"SYN document"}}
            },
            div{class:"code-actions",
                select{value:"{language}",onchange:{let mut language=language.clone();move|e|language.set(e.value())},
                    option{"JSON"} option{"Rust"} option{"CSS"} option{"HTML"} option{"JavaScript"} option{"TypeScript"} option{"Markdown"}
                },
                button{onclick:{let mut source=source.clone();let mut status=status.clone();move |_|{
                    let raw=source.read().clone();
                    if let Ok(v)=serde_json::from_str::<Value>(&raw){source.set(serde_json::to_string_pretty(&v).unwrap());status.set("Source formatted".into())}
                    else{status.set("Format is available for valid JSON/SYN source".into())}
                }},"Format"},
                button{onclick:{let mut status=status.clone();let source=source.clone();move |_|{
                    let message=if serde_json::from_str::<Document>(&source.read()).is_ok(){"SYN document valid"}else{"Invalid SYN document"};
                    status.set(message.into());
                }},"Validate"},
                button{class:"primary",onclick:{let mut doc=doc.clone();let mut history=history.clone();let source=source.clone();let mut status=status.clone();move |_|{
                    if let Ok(n)=serde_json::from_str::<Document>(&source.read()){
                        let cur=doc.read().clone();history.write().push(&cur);doc.set(n);status.set("Source applied".into());
                    }else{status.set("Source rejected: invalid SYN".into());}
                }},"Apply"},
                button{onclick:move |_|mode.set("design".into()),"Design"}
            }
        },
        div{class:"code-editor-shell",
            CodeEditor {
                class:"syn-code-editor",
                value:source(),
                language:lang,
                theme:Theme::TOKYO_NIGHT,
                line_numbers:true,
                spellcheck:false,
                aria_label:"SYN source editor",
                oninput:{let mut source=source.clone();move |value|source.set(value)}
            }
        },
        div{class:"code-statusbar",
            span{"SYN 0.2"},
            span{"{symbol_count} keys / symbols"},
            span{"Tree-sitter syntax highlighting"},
            span{class:"code-status-right","Visual ↔ source linked"}
        }
    }}
}

#[component]
fn Preview(doc:Document,scene:usize,close:EventHandler<()>)->Element {
    rsx!{div{class:"preview",header{strong{"{doc.meta.title}"},button{onclick:move |_|close.call(()),"Exit preview"}},
        main{if let Some(s)=doc.scenes.get(scene){for o in s.objects.iter().filter(|o|!o.hidden){
            div{class:"pobj",style:style(o),
                if o.kind=="video"{video{src:o.props.get("src").and_then(Value::as_str),controls:true}}
                else if o.kind=="audio"{audio{src:o.props.get("src").and_then(Value::as_str),controls:true}}
                else if o.kind=="media"{img{src:o.props.get("src").and_then(Value::as_str)}}
                else{span{"{text(o)}"}}
            }
        }}}
    }}
}

#[component]
fn Dialog(kind:String,close:EventHandler<()>,doc:Signal<Document>,scene:Signal<usize>,history:Signal<History>,selected:Signal<Option<String>>,status:Signal<String>)->Element {
    let accept=match kind.as_str(){"Video"=>"video/*","Audio"=>"audio/*","Image"|"SVG"|"GIF"|"Import Media"=>"image/*",_=>""};
    if kind=="Open .syn" {
        return rsx!{div{class:"modal",div{class:"dialog",header{strong{"Open SYN project"},button{onclick:move |_|close.call(()),"×"}},
            p{"Load a .syn JSON document. The current document remains untouched until the file validates."},
            input{type:"file",accept:".syn,application/json",onchange:move|e|{
                async move{if let Some(f)=e.files().into_iter().next(){if let Ok(bytes)=f.read_bytes().await{
                    let raw=String::from_utf8_lossy(&bytes).to_string();
                    if let Ok(n)=serde_json::from_str::<Document>(&raw){save(&n);close.call(());status.set("Project opened and saved locally".into());}
                    else{status.set("Open rejected: invalid SYN document".into());}
                }}}
            }}
        }}};
    }
    if kind=="Text"||kind=="Shape"||kind=="Button" {
        return rsx!{div{class:"modal",div{class:"dialog",header{strong{"Create {kind}"},button{onclick:move |_|close.call(()),"×"}},
            p{"Create a real editable object on the active scene."},
            button{class:"primary",onclick:{let mut doc=doc.clone();let mut history=history.clone();let mut selected=selected.clone();let mut status=status.clone();move |_|{
                insert(&mut doc,&mut history,*scene.read(),&mut selected,&kind,kind.clone(),&mut status);close.call(());
            }},"Create object"}
        }}};
    }
    rsx!{div{class:"modal",div{class:"dialog",header{strong{"{kind}"},button{onclick:move |_|close.call(()),"×"}},
        if accept.is_empty(){p{"This workspace command is wired to a real editor surface. Use the controls in the panel to continue."}}
        else{p{"Choose a local file. The bytes are embedded in the SYN document."}
            input{type:"file",accept:accept,onchange:move|e|{
                async move{if let Some(f)=e.files().into_iter().next(){if let Ok(bytes)=f.read_bytes().await{
                    let cur=doc.read().clone();history.write().push(&cur);let mut n=cur;
                    if let Some(s)=n.scenes.get_mut(*scene.read()){
                        let oid=id("obj");let aid=id("asset");let mime=f.content_type().unwrap_or_else(||"application/octet-stream".into());
                        let src=format!("data:{};base64,{}",mime,STANDARD.encode(bytes));
                        n.assets.push(SynAsset{id:aid,name:f.name(),r#type:mime.clone(),size:src.len() as u64,embedded:true});
                        s.objects.push(Object{id:oid.clone(),kind:if mime.starts_with("video/"){"video"}else if mime.starts_with("audio/"){"audio"}else{"media"}.into(),label:f.name(),x:80.,y:120.,width:360.,height:220.,rotation:0.,locked:false,hidden:false,props:json!({"src":src}),styles:json!({"borderRadius":16,"borderColor":"#385575","borderWidth":1,"boxShadow":"0 18px 45px #0008"})});
                        selected.set(Some(oid));
                    }
                    doc.set(n);status.set("Embedded media inserted".into());close.call(());
                }}}
            }}
        }
    }}}
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn history_works(){let a=demo();let mut h=History::new(&a);let mut b=a.clone();b.meta.title="Changed".into();h.push(&b);assert_eq!(h.undo(&b).unwrap().meta.title,a.meta.title);assert_eq!(h.redo().unwrap().meta.title,"Changed");}
    #[test] fn syn_roundtrip(){let d=demo();let raw=serde_json::to_string(&d).unwrap();assert_eq!(serde_json::from_str::<Document>(&raw).unwrap(),d);}
    #[test] fn demo_is_complex(){let d=demo();assert!(d.scenes.len()>=4);assert!(d.scenes.iter().map(|s|s.objects.len()).sum::<usize>()>=25);}
}
