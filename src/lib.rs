use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use uuid::Uuid;
use wasm_bindgen::prelude::*;

fn uid(prefix: &str) -> String {
    format!("{}-{}", prefix, &Uuid::new_v4().simple().to_string()[..8])
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub syn: String,
    pub title: String,
    pub scenes: Vec<Scene>,
    pub active_scene: usize,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scene {
    pub id: String,
    pub name: String,
    pub width: f32,
    pub height: f32,
    pub objects: Vec<Object>,
    pub animation: Animation,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Animation {
    pub duration: f32,
    pub fps: u32,
    pub tracks: Vec<Track>,
    pub looped: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track {
    pub id: String,
    pub target: String,
    pub property: String,
    pub keyframes: Vec<Keyframe>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keyframe {
    pub id: String,
    pub time: f32,
    pub value: f32,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Object {
    pub id: String,
    pub kind: String,
    pub label: String,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub rotation: f32,
    pub opacity: f32,
    pub locked: bool,
    pub hidden: bool,
    pub props: Value,
}

fn object(kind: &str, label: &str, x: f32, y: f32, w: f32, h: f32) -> Object {
    Object {
        id: uid(kind), kind: kind.into(), label: label.into(), x, y, width: w, height: h,
        rotation: 0.0, opacity: 1.0, locked: false, hidden: false,
        props: json!({"color":"#111827","textColor":"#f4f7fb","accent":"#9b8dff"}),
    }
}

impl Document {
    pub fn sample() -> Self {
        let mut s = Scene {
            id: "scene-01".into(), name: "Command Center".into(), width: 1200.0, height: 720.0,
            objects: vec![], animation: Animation { duration: 8.0, fps: 60, tracks: vec![], looped: true },
        };
        let mut bg = object("frame", "", 0.0, 0.0, 1200.0, 720.0);
        bg.props = json!({"color":"#0a0e16","gradient":"#18243c","radius":30});
        s.objects.push(bg);
        let mut orb = object("orb", "", 800.0, 70.0, 280.0, 280.0);
        orb.props = json!({"color":"#6e62ff","accent":"#67dff5","blur":70});
        s.objects.push(orb);
        let mut eyebrow = object("text", "SYN / CREATIVE SYSTEMS", 72.0, 72.0, 500.0, 32.0);
        eyebrow.props = json!({"fontSize":13,"color":"#8ea1bf","weight":700,"letterSpacing":2.8});
        s.objects.push(eyebrow);
        let mut title = object("text", "Design systems that move.", 72.0, 125.0, 690.0, 110.0);
        title.props = json!({"fontSize":54,"color":"#f5f7fb","weight":800,"lineHeight":1.05});
        s.objects.push(title);
        let mut copy = object("text", "Compose scenes, motion, media and code from one visual workspace.", 76.0, 252.0, 540.0, 70.0);
        copy.props = json!({"fontSize":18,"color":"#a6b3c7","lineHeight":1.5});
        s.objects.push(copy);
        let mut cta = object("button", "OPEN COMMAND CENTER", 76.0, 350.0, 224.0, 56.0);
        cta.props = json!({"color":"#9b8dff","textColor":"#080b11","radius":16,"weight":800});
        s.objects.push(cta);
        let mut secondary = object("button", "VIEW MOTION", 314.0, 350.0, 170.0, 56.0);
        secondary.props = json!({"color":"#182235","textColor":"#e7edf7","radius":16});
        s.objects.push(secondary);
        for (label, x, accent) in [("DESIGN",76.0,"#9b8dff"),("MOTION",352.0,"#67dff5"),("CODE",628.0,"#79e6b0")] {
            let mut c = object("card", label, x, 480.0, 260.0, 120.0);
            c.props = json!({"color":"#111a29","accent":accent,"radius":20});
            s.objects.push(c);
        }
        let mut stat = object("card", "60 FPS / READY", 910.0, 475.0, 210.0, 125.0);
        stat.props = json!({"color":"#101827","accent":"#df78ef","radius":20});
        s.objects.push(stat);
        let orb_id = s.objects[1].id.clone();
        s.animation.tracks.push(Track {
            id: uid("track"), target: orb_id, property: "x".into(),
            keyframes: vec![
                Keyframe{id:uid("key"),time:0.0,value:800.0}, Keyframe{id:uid("key"),time:2.0,value:900.0},
                Keyframe{id:uid("key"),time:4.0,value:800.0}, Keyframe{id:uid("key"),time:6.0,value:700.0},
                Keyframe{id:uid("key"),time:8.0,value:800.0},
            ],
        });
        Document { syn:"0.5".into(), title:"SYN Studio / Creative Systems".into(), scenes:vec![s], active_scene:0 }
    }
}

#[wasm_bindgen]
pub struct Engine {
    document: Document,
    history: Vec<Document>,
    future: Vec<Document>,
}

#[wasm_bindgen]
impl Engine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Engine {
        Engine { document: Document::sample(), history: vec![], future: vec![] }
    }
    pub fn document_json(&self) -> String {
        serde_json::to_string(&self.document).unwrap_or_else(|_| "{}".into())
    }
    pub fn load_json(&mut self, input: &str) -> Result<(), JsValue> {
        let doc: Document = serde_json::from_str(input)
            .map_err(|e| JsValue::from_str(&format!("Invalid SYN document: {e}")))?;
        Self::validate(&doc).map_err(|e| JsValue::from_str(&e))?;
        self.history.push(self.document.clone());
        self.document = doc;
        self.future.clear();
        Ok(())
    }
    pub fn validate_json(&self, input: &str) -> String {
        match serde_json::from_str::<Document>(input) {
            Ok(doc) => match Self::validate(&doc) {
                Ok(()) => json!({"ok":true,"message":"Valid SYN document"}).to_string(),
                Err(e) => json!({"ok":false,"message":e}).to_string(),
            },
            Err(e) => json!({"ok":false,"message":format!("Parse error: {e}")}).to_string(),
        }
    }
    pub fn command(&mut self, input: &str) -> Result<String, JsValue> {
        let command: Value = serde_json::from_str(input)
            .map_err(|e| JsValue::from_str(&format!("Invalid command: {e}")))?;
        let kind = command.get("type").and_then(Value::as_str).unwrap_or("");
        if kind == "undo" { self.undo(); return Ok(self.document_json()); }
        if kind == "redo" { self.redo(); return Ok(self.document_json()); }
        self.history.push(self.document.clone());
        self.future.clear();
        match self.apply(&command) {
            Ok(()) => Ok(self.document_json()),
            Err(e) => { self.history.pop(); Err(JsValue::from_str(&e)) }
        }
    }
    pub fn can_undo(&self) -> bool { !self.history.is_empty() }
    pub fn can_redo(&self) -> bool { !self.future.is_empty() }

    fn undo(&mut self) { if let Some(p)=self.history.pop(){self.future.push(self.document.clone());self.document=p;} }
    fn redo(&mut self) { if let Some(n)=self.future.pop(){self.history.push(self.document.clone());self.document=n;} }

    fn apply(&mut self, c: &Value) -> Result<(), String> {
        let scene = self.document.active_scene;
        match c.get("type").and_then(Value::as_str).unwrap_or("") {
            "add_object" => {
                let kind=c.get("kind").and_then(Value::as_str).unwrap_or("card");
                let label=c.get("label").and_then(Value::as_str).unwrap_or("New object");
                let x=c.get("x").and_then(Value::as_f64).unwrap_or(180.0) as f32;
                let y=c.get("y").and_then(Value::as_f64).unwrap_or(160.0) as f32;
                let w=c.get("width").and_then(Value::as_f64).unwrap_or(240.0) as f32;
                let h=c.get("height").and_then(Value::as_f64).unwrap_or(80.0) as f32;
                let mut o=object(kind,label,x,y,w,h); if let Some(p)=c.get("props"){o.props=p.clone();}
                self.document.scenes[scene].objects.push(o);
            }
            "delete_object" => {
                let id=c.get("id").and_then(Value::as_str).ok_or("Object id is required")?;
                self.document.scenes[scene].objects.retain(|o|o.id!=id);
            }
            "set_object" => {
                let id=c.get("id").and_then(Value::as_str).ok_or("Object id is required")?;
                let o=self.document.scenes[scene].objects.iter_mut().find(|o|o.id==id).ok_or("Object not found")?;
                if let Some(v)=c.get("label").and_then(Value::as_str){o.label=v.into();}
                for (field,slot) in [("x",&mut o.x),("y",&mut o.y),("width",&mut o.width),("height",&mut o.height),("rotation",&mut o.rotation),("opacity",&mut o.opacity)] {
                    if let Some(v)=c.get(field).and_then(Value::as_f64){*slot=v as f32;}
                }
                if let Some(p)=c.get("props").and_then(Value::as_object) {
                    let map=o.props.as_object_mut().ok_or("Object properties are not an object")?;
                    for (k,v) in p {map.insert(k.clone(),v.clone());}
                }
            }
            "add_scene" => {
                let n=self.document.scenes.len()+1;
                self.document.scenes.push(Scene{id:uid("scene"),name:format!("Scene {n:02}"),width:1200.0,height:720.0,objects:vec![],animation:Animation{duration:8.0,fps:60,tracks:vec![],looped:true}});
                self.document.active_scene=n-1;
            }
            "select_scene" => {
                let i=c.get("index").and_then(Value::as_u64).unwrap_or(0) as usize;
                if i>=self.document.scenes.len(){return Err("Scene index out of range".into());}
                self.document.active_scene=i;
            }
            "rename_scene" => {
                let n=c.get("name").and_then(Value::as_str).unwrap_or("").trim();
                if n.is_empty(){return Err("Scene name cannot be empty".into());}
                self.document.scenes[scene].name=n.into();
            }
            "delete_scene" => {
                if self.document.scenes.len()<=1{return Err("A project needs at least one scene".into());}
                self.document.scenes.remove(scene);
                self.document.active_scene=self.document.active_scene.min(self.document.scenes.len()-1);
            }
            "set_title" => self.document.title=c.get("title").and_then(Value::as_str).unwrap_or("SYN Studio").into(),
            _ => return Err(format!("Unknown command: {}", c.get("type").and_then(Value::as_str).unwrap_or(""))),
        }
        Self::validate(&self.document)
    }
    fn validate(doc:&Document)->Result<(),String>{
        if doc.scenes.is_empty(){return Err("A SYN document needs at least one scene".into());}
        if doc.active_scene>=doc.scenes.len(){return Err("Active scene is out of range".into());}
        for s in &doc.scenes {
            if s.width<=0.0||s.height<=0.0{return Err(format!("Scene '{}' has invalid dimensions",s.name));}
            let mut ids=std::collections::HashSet::new();
            for o in &s.objects {if o.id.trim().is_empty(){return Err("Object id cannot be empty".into());}if !ids.insert(&o.id){return Err(format!("Duplicate object id '{}'",o.id));}}
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn sample_is_valid(){assert!(Engine::validate(&Document::sample()).is_ok());}
    #[test] fn command_changes_document(){let mut e=Engine::new();let before=e.document_json();e.command(r#"{"type":"add_object","kind":"text","label":"Hello"}"#).unwrap();assert_ne!(before,e.document_json());assert!(e.can_undo());e.command(r#"{"type":"undo"}"#).unwrap();assert_eq!(before,e.document_json());}
}
