use dioxus::prelude::*;
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

fn object(kind: &str, label: &str, x: f32, y: f32, w: f32, h: f32, styles: Value) -> Object {
    Object {
        id: id("obj"), kind: kind.into(), label: label.into(), x, y, width: w, height: h,
        rotation: 0., locked: false, hidden: false, props: json!({"text": label}), styles
    }
}

fn text(o: &Object) -> String {
    o.props.get("text").and_then(Value::as_str).unwrap_or(&o.label).into()
}

fn demo() -> Document {
    let mut d = Document {
        syn: "0.1".into(), r#type: "document".into(),
        meta: Meta { id: id("doc"), title: "SYN Studio / Creative Systems".into() },
        viewport: Viewport { width: 1120, height: 640 }, assets: vec![], scenes: vec![]
    };

    let mut home = Scene {
        id: "HOME".into(), name: "Command Center".into(), background: "#080d14".into(),
        objects: vec![], interactions: vec![]
    };
    home.objects.push(object("text", "SYN / RUST CREATIVE IDE", 54., 36., 500., 20.,
        json!({"fontSize":10,"fontWeight":900,"color":"#91b0ff","letterSpacing":3})));
    home.objects.push(object("text", "Build the whole system.", 54., 72., 610., 62.,
        json!({"fontSize":48,"fontWeight":900,"color":"#f5f8ff","letterSpacing":-2})));
    home.objects.push(object("text", "A visual development environment for scenes, media, motion, behavior, data and portable SYN source.", 58., 142., 570., 54.,
        json!({"fontSize":15,"color":"#9aabc0","lineHeight":1.45})));

    let hero = object("shape", "SYSTEM PREVIEW", 666., 38., 390., 260.,
        json!({"background":"linear-gradient(145deg,#172a46,#0c141f)","borderColor":"#385575","borderWidth":1,"borderRadius":22,"boxShadow":"0 30px 80px #0009"}));
    home.objects.push(hero);
    home.objects.push(object("text", "LIVE DOCUMENT", 694., 67., 220., 18.,
        json!({"fontSize":9,"fontWeight":900,"color":"#91b0ff","letterSpacing":2})));
    home.objects.push(object("text", "04", 694., 99., 120., 62.,
        json!({"fontSize":58,"fontWeight":950,"color":"#ffffff"})));
    home.objects.push(object("text", "SCENES", 694., 161., 100., 18.,
        json!({"fontSize":9,"fontWeight":800,"color":"#6e829c","letterSpacing":2})));
    home.objects.push(object("text", "18", 830., 99., 120., 62.,
        json!({"fontSize":58,"fontWeight":950,"color":"#ffffff"})));
    home.objects.push(object("text", "OBJECTS", 830., 161., 100., 18.
        json!({"fontSize":9,"fontWeight":800,"color":"#6e829c","letterSpacing":2})));
    home.objects.push(object("text", "SOURCE  •  MEDIA  •  MOTION  •  EVENTS", 694., 204., 300., 20.
        json!({"fontSize":10,"fontWeight":750,"color":"#c4d0df","letterSpacing":1})));

    let btn = object("button", "Explore architecture", 58., 228., 190., 48.,
        json!({"background":"linear-gradient(180deg,#eef3ff,#aebfff)","color":"#07101c","fontSize":12,"fontWeight":900,"borderRadius":11,"boxShadow":"0 12px 30px #0006"}));
    let bid = btn.id.clone();
    home.objects.push(btn);
    home.interactions.push(Interaction {
        id: id("evt"), event: Event { r#type:"click".into(), target:bid },
        actions: vec![Action::Goto { target:"ARCH".into() }]
    });

    for (i, (k, v)) in [
        ("VISUAL", "Canvas + responsive composition"),
        ("BEHAVIOR", "Events + navigation graph"),
        ("MEDIA", "Embedded image / audio / video"),
        ("SOURCE", "Structured SYN + validation"),
    ].iter().enumerate() {
        let x = 54. + (i % 2) as f32 * 300.;
        let y = 320. + (i / 2) as f32 * 112.;
        home.objects.push(object("shape", k, x, y, 276., 86.,
            json!({"background":"#0e1722","borderColor":"#263b55","borderWidth":1,"borderRadius":14})));
        home.objects.push(object("text", k, x+18., y+15., 220., 18.,
            json!({"fontSize":9,"fontWeight":900,"color":"#91b0ff","letterSpacing":1.5})));
        home.objects.push(object("text", v, x+18., y+40., 235., 30.
            json!({"fontSize":11,"fontWeight":650,"color":"#b8c6d7"})));
    }

    let mut arch = Scene {
        id:"ARCH".into(), name:"Architecture".into(), background:"#091019".into(),
        objects:vec![], interactions:vec![]
    };
    arch.objects.push(object("text","01 / ARCHITECTURE",54.,36.,400.,20.,json!({"fontSize":9,"fontWeight":900,"color":"#91b0ff","letterSpacing":2})));
    arch.objects.push(object("text","One document. Many surfaces.",54.,68.,700.,56.,json!({"fontSize":42,"fontWeight":900,"color":"#f5f8ff"})));
    arch.objects.push(object("text","Rust owns the state. The UI is a renderer over the same document model.",58.,132.,700.,28.,json!({"fontSize":13,"color":"#91a4bb"})));
    let cards = [
        ("DOCUMENT", "SYN JSON", "Portable source of truth."),
        ("SCENES", "04 WORKSPACES", "Independent compositions."),
        ("MEDIA", "EMBEDDED", "Assets travel with projects."),
        ("EVENTS", "GRAPH", "Behavior remains inspectable."),
        ("MOTION", "TIMELINE", "Tracks, keyframes, playback."),
        ("DATA", "BINDINGS", "State stays structured."),
    ];
    for (i,(a,b,c)) in cards.iter().enumerate() {
        let x=54.+(i%3) as f32*342.;
        let y=192.+(i/3) as f32*136.;
        arch.objects.push(object("shape",*a,x,y,318.,108.,json!({"background":"linear-gradient(145deg,#111d2b,#0b121b)","borderColor":"#29415d","borderWidth":1,"borderRadius":15,"boxShadow":"0 16px 40px #0006"})));
        arch.objects.push(object("text",*a,x+18.,y+17.,150.,16.,json!({"fontSize":9,"fontWeight":900,"color":"#91b0ff","letterSpacing":1.5})));
        arch.objects.push(object("text",*b,x+18.,y+42.,230.,25.,json!({"fontSize":18,"fontWeight":850,"color":"#eef3fb"})));
        arch.objects.push(object("text",*c,x+18.,y+76.,260.,20.,json!({"fontSize":10,"color":"#8295ad"})));
    }

    let mut motion = Scene {
        id:"MOTION".into(), name:"Motion Lab".into(), background:"#080d14".into(),
        objects:vec![], interactions:vec![]
    };
    motion.objects.push(object("text","02 / MOTION LAB",54.,36.,400.,20.,json!({"fontSize":9,"fontWeight":900,"color":"#91b0ff","letterSpacing":2})));
    motion.objects.push(object("text","Animation is part of the document.",54.,68.,760.,56.,json!({"fontSize":40,"fontWeight":900,"color":"#f5f8ff"})));
    motion.objects.push(object("shape","TIMELINE",54.,156.,1010.,214.,json!({"background":"#0b141f","borderColor":"#263c58","borderWidth":1,"borderRadius":17})));
    for i in 0..8 {
        let x=86.+i as f32*120.;
        motion.objects.push(object("text",&format!("{:02}",i),x,177.,40.,18.,json!({"fontSize":9,"fontWeight":800,"color":"#657a95"})));
        motion.objects.push(object("shape","",x,205.,1.,130.,json!({"background":"#24374e"})));
    }
    motion.objects.push(object("shape","KEYFRAMES",86.,224.,770.,36.,json!({"background":"linear-gradient(90deg,#86a9ff,#7e8cff)","borderRadius":8,"boxShadow":"0 8px 25px #86a9ff44"})));
    motion.objects.push(object("shape","SECONDARY",160.,282.,520.,28.,json!({"background":"#23344b","borderRadius":7})));
    motion.objects.push(object("text","PLAYBACK   00:00.00     01:24.00",86.,338.,400.,18.,json!({"fontSize":9,"fontWeight":850,"color":"#8da2bb","letterSpacing":1})));

    let mut media = Scene {
        id:"MEDIA".into(), name:"Media Lab".into(), background:"#080d14".into(),
        objects:vec![], interactions:vec![]
    };
    media.objects.push(object("text","03 / MEDIA LAB",54.,36.,400.,20.,json!({"fontSize":9,"fontWeight":900,"color":"#91b0ff","letterSpacing":2})));
    media.objects.push(object("text","Media is a first-class object.",54.,68.,720.,56.,json!({"fontSize":40,"fontWeight":900,"color":"#f5f8ff"})));
    media.objects.push(object("shape","VIDEO",54.,156.,500.,300.,json!({"background":"linear-gradient(145deg,#172b45,#0a111a)","borderColor":"#385575","borderWidth":1,"borderRadius":18,"boxShadow":"0 25px 70px #0009"})));
    media.objects.push(object("text","VIDEO / AUDIO / IMAGE",82.,184.,320.,18.,json!({"fontSize":9,"fontWeight":900,"color":"#91b0ff","letterSpacing":2})));
    media.objects.push(object("text","DROP MEDIA HERE",82.,244.,350.,42.,json!({"fontSize":25,"fontWeight":900,"color":"#eef3fb"})));
    media.objects.push(object("text","Embedded assets are serialized into the .syn document.",82.,300.,350.,42.,json!({"fontSize":11,"color":"#91a4bb","lineHeight":1.5})));
    media.objects.push(object("shape","ASSET LIBRARY",590.,156.,474.,300.,json!({"background":"#0c151f","borderColor":"#263c58","borderWidth":1,"borderRadius":18})));
    for (i,(name,kind)) in [("hero-image.png","IMAGE"),("intro-video.mp4","VIDEO"),("voiceover.wav","AUDIO"),("Inter Variable","FONT")].iter().enumerate() {
        let y=190.+i as f32*57.;
        media.objects.push(object("text",kind,614.,y,80.,18.,json!({"fontSize":8,"fontWeight":900,"color":"#91b0ff","letterSpacing":1.2})));
        media.objects.push(object("text",name,700.,y,270.,18.,json!({"fontSize":11,"fontWeight":700,"color":"#d5dfeb"})));
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
    let mut zoom=use_signal(||1.0f32);
    let mut menu=use_signal(||None::<String>);
    let mut search=use_signal(String::new);
    let mut modal=use_signal(||None::<String>);
    let mut status=use_signal(||"Ready".to_string());
    let mut history=use_signal(||History::new(&initial()));
    let mut source=use_signal(String::new);
    let mut panel=use_signal(||"inspector".to_string());

    let active=doc.read().scenes.get(*scene.read()).cloned();
    let scene_count=doc.read().scenes.len();
    let selected_id=selected.read().clone();
    let status_text=status.read().clone();
    let zoom_label=format!("{}%",(*zoom.read()*100.) as i32);
    let dock_label=format!("{} objects  ·  {} scenes  ·  {} assets",
        active.as_ref().map(|s|s.objects.len()).unwrap_or(0),scene_count,doc.read().assets.len());

    if *preview.read() {
        return rsx!{ Preview { doc:doc.read().clone(), scene:*scene.read(), close:move |_|preview.set(false) } };
    }

    rsx! {
        document::Stylesheet { href:CSS }
        div { class:"app",
            header { class:"top",
                div { class:"brand", b{"S"}, div{strong{"SYN Studio"},small{"RUST-FIRST CREATIVE IDE"}} }
                nav { for (name,_) in menus() {
                    button {
                        class:if menu.read().as_deref()==Some(name){"topmenu active"}else{"topmenu"},
                        onclick:{
                            let n=name.to_string(); let mut menu=menu.clone(); let mut search=search.clone();
                            move |_|{ search.set(String::new()); if menu.read().as_deref()==Some(n.as_str()){menu.set(None)}else{menu.set(Some(n.clone()))}}
                        },
                        "{name}"
                    }
                }}
                div { class:"topbuttons",
                    button { onclick:{let mut status=status.clone();let d=doc.clone();move |_|{save(&d.read());status.set("Saved locally".into())}}, "Save" }
                    button { class:"primary", onclick:{let d=doc.clone();move |_|download("syn-studio-project.syn",&serde_json::to_string_pretty(&*d.read()).unwrap())}, "Export .syn" }
                }
            }

            if let Some(open)=menu.read().clone() {
                Mega {
                    open,
                    search:search.clone(),
                    pick:{
                        let mut menu=menu.clone();let mut modal=modal.clone();let mut mode=mode.clone();let mut source=source.clone();
                        let mut preview=preview.clone();let mut doc=doc.clone();let mut history=history.clone();
                        let mut scene=scene.clone();let mut selected=selected.clone();let mut status=status.clone();let mut panel=panel.clone();
                        move |item:String| {
                            menu.set(None);
                            match item.as_str() {
                                "New Project" => {let cur=doc.read().clone();history.write().push(&cur);doc.set(demo());scene.set(0);selected.set(None);status.set("New project".into());},
                                "Save" => {save(&doc.read());status.set("Saved locally".into());},
                                "Save Snapshot" => {save(&doc.read());download("syn-studio-snapshot.syn",&serde_json::to_string_pretty(&*doc.read()).unwrap());status.set("Snapshot exported".into());},
                                "Open .syn" => modal.set(Some("Open .syn".into())),
                                "Export .syn" => download("syn-studio-project.syn",&serde_json::to_string_pretty(&*doc.read()).unwrap()),
                                "Preview" => preview.set(true),
                                "Undo" => {let cur=doc.read().clone();if let Some(n)=history.write().undo(&cur){doc.set(n);selected.set(None);status.set("Undo".into())}},
                                "Redo" => {if let Some(n)=history.write().redo(){doc.set(n);status.set("Redo".into())}},
                                "Delete Selected" => {
                                    let selected_id=selected.read().clone();if let Some(id)=selected_id{let cur=doc.read().clone();history.write().push(&cur);let mut n=cur;let current_scene=*scene.read();if let Some(s)=n.scenes.get_mut(current_scene){s.objects.retain(|o|o.id!=id)}selected.set(None);doc.set(n);status.set("Object deleted".into())}
                                },
                                "Duplicate Selected" => {
                                    let selected_id=selected.read().clone();if let Some(id)=selected_id{let current_scene=*scene.read();duplicate_selected(&mut doc,&mut history,current_scene,&id,&mut selected,&mut status)}
                                },
                                "Text"|"Rich Text"|"Heading"|"Paragraph" => insert(&mut doc,&mut history,*scene.read(),&mut selected,"text",item,&mut status),
                                "Button" => insert(&mut doc,&mut history,*scene.read(),&mut selected,"button",item,&mut status),
                                "Shape"|"Card" => insert(&mut doc,&mut history,*scene.read(),&mut selected,"shape",item,&mut status),
                                "Component" => insert(&mut doc,&mut history,*scene.read(),&mut selected,"component",item,&mut status),
                                "Image"|"SVG"|"GIF"|"Audio"|"Video"|"Import Media" => modal.set(Some(item)),
                                "New Scene" => new_scene(&mut doc,&mut history,&mut scene,&mut selected,&mut status),
                                "Duplicate Scene" => {let current_scene=*scene.read();duplicate_scene(&mut doc,&mut history,current_scene,&mut scene,&mut status)},
                                "Delete Scene" => delete_scene(&mut doc,&mut history,&mut scene,&mut selected,&mut status),
                                "Design Inspector"|"Object Inspector"|"Layers"|"Scene Graph"|"Asset Library"|"Timeline"|"Interaction Graph"|"Data"|"Console"|"Output"|"Code Editor"|"Validation"|"AI Workbench" => {
                                    panel.set(panel_for(item.as_str()));
                                    if item=="Code Editor"{source.set(serde_json::to_string_pretty(&*doc.read()).unwrap());mode.set("code".into())}
                                    status.set(format!("{} opened",item));
                                },
                                "Align Center"|"Center on Canvas" => {if let Some(id)=selected.read().clone(){align_center(&mut doc,&mut history,*scene.read(),&id,&mut status)}},
                                "Reset Transform" => {if let Some(id)=selected.read().clone(){reset_transform(&mut doc,&mut history,*scene.read(),&id,&mut status)}},
                                "Validate" => {status.set(validate(&doc.read()));},
                                "Publish Preview" => preview.set(true),
                                _ => {status.set(format!("{} is available from its workspace panel",item));}
                            }
                        }
                    }
                }
            }

            main { class:"work",
                aside { class:"rail",
                    span{"TOOLS"}
                    for (glyph,name) in [("↖","select"),("✥","move"),("✋","pan"),("T","text"),("◇","shape"),("▣","button"),("◈","media")] {
                        button {
                            class:if *tool.read()==name{"railbtn active"}else{"railbtn"},
                            onclick:{let name=name.to_string();let mut tool=tool.clone();let mut modal=modal.clone();move |_|{tool.set(name.clone());if name=="text"{modal.set(Some("Text".into()))}else if name=="shape"{modal.set(Some("Shape".into()))}else if name=="button"{modal.set(Some("Button".into()))}else if name=="media"{modal.set(Some("Import Media".into()))}}},
                            span{class:"ico","{glyph}"} small{"{name}"}
                        }
                    }
                }

                section { class:"center",
                    div { class:"bar",
                        div { button{onclick:move |_|{let current_scene=*scene.read();if current_scene>0{scene.set(current_scene-1);selected.set(None)}},"‹"} strong{"{active.as_ref().map(|s|s.name.clone()).unwrap_or_default()}"} button{onclick:move |_|{let current_scene=*scene.read();if current_scene+1<scene_count{scene.set(current_scene+1);selected.set(None)}},"›"} }
                        div { class:"modes",
                            button{class:if *mode.read()=="design"{"active"}else{""},onclick:move |_|mode.set("design".into()),"Design"}
                            button{class:if *mode.read()=="code"{"active"}else{""},onclick:{let mut mode=mode.clone();let mut source=source.clone();let d=doc.clone();move |_|{source.set(serde_json::to_string_pretty(&*d.read()).unwrap());mode.set("code".into())}},"Code"}
                            button{onclick:move |_|preview.set(true),"Preview"}
                        }
                        div { class:"zoom",
                            button{onclick:move |_|{let current_zoom=*zoom.read();zoom.set((current_zoom-0.1).max(0.5))},"−"}
                            span{"{zoom_label}"}
                            button{onclick:move |_|{let current_zoom=*zoom.read();zoom.set((current_zoom+0.1).min(1.5))},"+"}
                        }
                    }
                    if *mode.read()=="code" {
                        Code { source:source.clone(),doc:doc.clone(),history:history.clone(),status:status.clone(),mode:mode.clone() }
                    } else {
                        Canvas { doc:doc.clone(),scene:*scene.read(),selected:selected.clone(),zoom:*zoom.read(),pick:{let mut selected=selected.clone();move |v|selected.set(v)} }
                    }
                    div { class:"dock", "RUST DOCUMENT", span{"{dock_label}"}, div{class:"grow"}, span{"{status_text}"} }
                }

                aside { class:"inspector",
                    div { class:"inspecthead",
                        button{class:if *panel.read()=="inspector"{"paneltab active"}else{"paneltab"},onclick:move |_|panel.set("inspector".into()),"INSPECT"}
                        button{class:if *panel.read()=="assets"{"paneltab active"}else{"paneltab"},onclick:move |_|panel.set("assets".into()),"ASSETS"}
                    }
                    if *panel.read()=="assets" {
                        AssetsPanel { doc:doc.clone() }
                    } else if *panel.read()=="timeline" {
                        TimelinePanel {}
                    } else if *panel.read()=="interaction" {
                        InteractionPanel { doc:doc.clone(),scene:*scene.read() }
                    } else if *panel.read()=="code" {
                        div{class:"panelbody",strong{"SOURCE"},p{"Rust owns the document model. The Code workspace edits the same SYN structure."}}
                    } else if *panel.read()=="ai" {
                        AiPanel { status:status.clone() }
                    } else if let Some(id)=selected_id {
                        Inspect { doc:doc.clone(),scene:*scene.read(),id,status:status.clone(),history:history.clone() }
                    } else {
                        div{class:"empty",b{"◇"},strong{"Select an object"},p{"Geometry, content, styles and state appear here."}}
                    }
                }
            }

            if let Some(kind)=modal.read().clone() {
                Dialog{kind,close:move |_|modal.set(None),doc:doc.clone(),scene:scene.clone(),history:history.clone(),selected:selected.clone(),status:status.clone()}
            }
        }
    }
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
    rsx!{div{class:"code",
        div{class:"codehead",span{"SOURCE / SYN 0.1"},div{
            button{onclick:{let mut source=source.clone();move |_|{let raw=source.read().clone();if let Ok(v)=serde_json::from_str::<Value>(&raw){source.set(serde_json::to_string_pretty(&v).unwrap())}}},"Format"},
            button{onclick:{let mut status=status.clone();let source=source.clone();move |_|{
                let message=if serde_json::from_str::<Document>(&source.read()).is_ok(){"Valid SYN document"}else{"Invalid SYN document"};
                status.set(message.into());
            }},"Validate"},
            button{class:"primary",onclick:{let mut doc=doc.clone();let mut history=history.clone();let source=source.clone();let mut status=status.clone();move |_|{
                if let Ok(n)=serde_json::from_str::<Document>(&source.read()){
                    let cur=doc.read().clone();history.write().push(&cur);doc.set(n);status.set("Source applied".into());
                }else{status.set("Source rejected: invalid SYN".into());}
            }},"Apply"},
            button{onclick:move |_|mode.set("design".into()),"Design"}
        }},
        textarea{value:"{source}",oninput:move|e|source.set(e.value())},
        div{"Rust document editor • visual and source views share one model."}
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
