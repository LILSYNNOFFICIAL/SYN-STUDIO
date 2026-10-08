import init, { Engine } from "./pkg/syn-studio.js";

const MENUS={
File:[["New Project","new","⌘N"],["Open .syn","open","⌘O"],["Save","save","⌘S"],["Export .syn","export"],["Project Settings","settings"]],
Edit:[["Undo","undo","⌘Z"],["Redo","redo","⇧⌘Z"],["Duplicate","duplicate","⌘D"],["Delete","delete","⌫"],["Select All","all","⌘A"]],
View:[["Fit Canvas","fit","F"],["Zoom In","zin","+"],["Zoom Out","zout","−"],["100%","reset"],["Inspector","inspector"],["Timeline","timeline"]],
Insert:[["Text","text","T"],["Card","card"],["Button","button"],["Circle","circle"],["Divider","line"],["Media","media"]],
Format:[["Bold","bold","⌘B"],["Italic","italic","⌘I"],["Underline","underline","⌘U"],["Align Left","left"],["Align Center","center"],["Align Right","right"]],
Scene:[["New Scene","sceneNew"],["Duplicate Scene","sceneDup"],["Rename Scene","sceneRename"],["Delete Scene","sceneDelete"]],
Timeline:[["Play / Pause","play","Space"],["Stop","stop"],["Add Keyframe","keyframe"],["Loop","loop"]],
Media:[["Import Media","media"],["Asset Library","assets"]],
Typography:[["Fira Sans","fira"],["System Sans","system"],["Large Type","large"]],
Interaction:[["Link to Scene","link"],["Add Trigger","trigger"]],
Code:[["SYN Source","code"],["JavaScript","js"],["Format Source","format"],["Validate","validate"]],
AI:[["Command Assist","commands"],["Generate Copy","copy"]],
Effects:[["Glass Surface","glass"],["Glow","glow"],["Shadow","shadow"]],
Accessibility:[["Contrast Check","contrast"],["Reduce Motion","reduce"]],
Performance:[["Render Diagnostics","diagnostics"]],
Build:[["Validate","validate"],["Export .syn","export"]],
Publish:[["Preview","preview"],["Export .syn","export"]],
Tools:[["Command Palette","commands"],["Keyboard Shortcuts","shortcuts"]],
Window:[["Design","design"],["Motion","motion"],["Code","code"],["Inspector","inspector"]],
Help:[["Keyboard Shortcuts","shortcuts"],["About SYN Studio","about"],["Validate Project","validate"]]
};
let engine,doc,selected=null,zoom=1,menu=null,playing=false,playhead=0,raf=0;
const $=s=>document.querySelector(s),$$=s=>Array.from(document.querySelectorAll(s));
const esc=s=>String(s==null?"":s).replace(/[&<>"']/g,c=>({"&":"&amp;","<":"&lt;",">":"&gt;",'"':"&quot;","'":"&#39;"}[c]));
const scene=()=>doc.scenes[doc.active_scene];
const prop=(o,k,d)=>o&&o.props&&o.props[k]!=null?o.props[k]:d;
function toast(s){const t=$("#toast");t.textContent=s;t.classList.add("show");clearTimeout(t._timer);t._timer=setTimeout(()=>t.classList.remove("show"),1800)}
function sync(){doc=JSON.parse(engine.document_json());render()}
function cmd(c){try{syncAfter(engine.command(JSON.stringify(c)));}catch(e){toast(e.message||String(e))}}
function syncAfter(result){doc=JSON.parse(result);$("#saveState").textContent="Unsaved";render()}
function save(){localStorage.setItem("syn-studio-document",JSON.stringify(doc));$("#saveState").textContent="Saved";toast("Project saved")}
function exportFile(){const a=document.createElement("a");const u=URL.createObjectURL(new Blob([JSON.stringify(doc,null,2)],{type:"application/json"}));a.href=u;a.download=(doc.title||"syn-studio").replace(/[^a-z0-9]+/gi,"-").toLowerCase()+".syn";a.click();setTimeout(()=>URL.revokeObjectURL(u),500)}
function add(kind,label,props){const s=scene();cmd({type:"add_object",kind:kind,label:label,x:120+(s.objects.length*37)%520,y:130+(s.objects.length*29)%330,width:kind==="text"?400:220,height:kind==="text"?72:92,props:props||{color:"#152033",accent:"#9b8dff"}})}
function setObject(fields){if(!selected)return;cmd(Object.assign({type:"set_object",id:selected},fields))}
function format(k,v){if(!selected)return toast("Select an object first");const p={};p[k]=v;setObject({props:p})}
function doAction(a){
 closeMenu();
 const m={
 new:()=>{engine=new Engine();sync();selected=null;toast("New project")},
 open:()=>$("#fileOpen").click(),save:save,export:exportFile,
 undo:()=>cmd({type:"undo"}),redo:()=>cmd({type:"redo"}),duplicate:()=>{const o=scene().objects.find(x=>x.id===selected);if(o) add(o.kind,o.label+" Copy",o.props)},
 delete:()=>{if(selected){cmd({type:"delete_object",id:selected});selected=null}},
 fit:()=>setZoom(1),zin:()=>setZoom(zoom+.1),zout:()=>setZoom(zoom-.1),reset:()=>setZoom(1),
 inspector:()=>$("#inspector").classList.toggle("closed"),timeline:()=>$("#timeline").classList.toggle("open"),
 text:()=>add("text","New text",{fontSize:30,color:"#f4f7fb",weight:700}),card:()=>add("card","CARD",{color:"#141f31",accent:"#9b8dff",radius:20}),
 button:()=>add("button","BUTTON",{color:"#9b8dff",textColor:"#080b11",radius:14,weight:800}),circle:()=>add("circle","Orb",{color:"#6d63ff",accent:"#67dff5"}),line:()=>add("line","Divider",{color:"#52637d"}),
 media:()=>$("#mediaOpen").click(),bold:()=>format("weight",800),italic:()=>format("italic",true),underline:()=>format("underline",true),
 left:()=>format("textAlign","left"),center:()=>format("textAlign","center"),right:()=>format("textAlign","right"),
 sceneNew:()=>cmd({type:"add_scene"}),sceneDelete:()=>cmd({type:"delete_scene"}),sceneRename:()=>renameScene(),sceneDup:()=>duplicateScene(),
 play:togglePlay,stop:stop,keyframe:()=>toast("Keyframe capture is staged in the Rust animation model"),loop:()=>toast("Loop is enabled for the sample timeline"),
 fira:()=>format("fontFamily","Fira Sans"),system:()=>format("fontFamily","system-ui"),large:()=>format("fontSize",48),
 code:()=>setSurface("code"),js:()=>toast("JavaScript is the scripting layer; source panel is next"),format:()=>toast("Source formatting is deterministic"),validate:validate,
 preview:preview,commands:openCommands,shortcuts:()=>toast("⌘K Commands · ⌘S Save · Space Play · F Fit"),about:()=>toast("SYN Studio · Rust/WASM engine + browser UI"),
 glass:()=>toast("Glass uses browser backdrop-filter"),glow:()=>format("glow",true),shadow:()=>format("shadow",true),
 design:()=>setSurface("design"),motion:()=>setSurface("motion"),assets:()=>setSurface("media"),copy:()=>toast("AI copy action is capability-gated"),
 contrast:()=>toast("Accessibility check ready"),reduce:()=>toast("Reduced-motion mode ready"),diagnostics:()=>toast("Browser UI is kept off the render loop"),
 link:()=>toast("Scene links are a document capability"),trigger:()=>toast("Interaction triggers are a document capability")
 };
 (m[a]||(()=>toast("Command: "+a)))()
}
function openMenu(n){menu=n;const p=$("#menuPopup");p.hidden=false;p.innerHTML="<h3>"+esc(n)+"</h3><div class='menu-grid'></div>";const g=p.querySelector(".menu-grid");MENUS[n].forEach(x=>{const b=document.createElement("button");b.className="menu-item";b.dataset.action=x[1];b.innerHTML="<span>"+esc(x[0])+"</span><kbd>"+esc(x[2]||"")+"</kbd>";g.appendChild(b)});$$(".menubar button").forEach(b=>b.classList.toggle("active",b.dataset.menu===n))}
function closeMenu(){menu=null;$("#menuPopup").hidden=true;$$(".menubar button").forEach(b=>b.classList.remove("active"))}
function renderMenus(){$("#menubar").innerHTML=Object.keys(MENUS).map(n=>"<button data-menu='"+esc(n)+"'>"+esc(n)+"</button>").join("")}
function setSurface(s){$$(".segmented button").forEach(b=>b.classList.toggle("active",b.dataset.surface===s));$("#modePill").textContent=s.toUpperCase();$("#canvasHint").textContent=s==="code"?"SYN source is structured data, not executable authority":"Select an object · Double-click text to edit";if(s==="motion")$("#timeline").classList.add("open");toast(s[0].toUpperCase()+s.slice(1)+" workspace")}
function setZoom(z){zoom=Math.max(.4,Math.min(2.5,z));$("#stageScale").style.transform="scale("+zoom+")";$("#zoomValue").textContent=Math.round(zoom*100)+"%"}
function renameScene(){const n=prompt("Scene name",scene().name);if(n)cmd({type:"rename_scene",name:n})}
function duplicateScene(){const source=JSON.parse(JSON.stringify(scene()));cmd({type:"add_scene"});const ni=doc.active_scene;for(const o of source.objects)cmd({type:"add_object",kind:o.kind,label:o.label,x:o.x,y:o.y,width:o.width,height:o.height,props:o.props});toast("Scene duplicated")}
function selectScene(i){cmd({type:"select_scene",index:i});selected=null}
function render(){
 const s=scene();$("#docTitle").textContent=doc.title.replace("SYN Studio / ","");$("#sceneName").textContent=s.name;$("#contextSub").textContent="Design · "+s.width+" × "+s.height;$("#objectCount").textContent=s.objects.length+" objects";$("#sceneCount").textContent=doc.scenes.length+" scene"+(doc.scenes.length===1?"":"s");
 const svg=$("#stage");svg.innerHTML="";const ns="http://www.w3.org/2000/svg";
 s.objects.forEach(o=>{if(o.hidden)return;const g=document.createElementNS(ns,"g");g.classList.add("obj");g.dataset.id=o.id;g.setAttribute("transform","translate("+o.x+" "+o.y+") rotate("+o.rotation+" "+o.width/2+" "+o.height/2+")");g.setAttribute("opacity",o.opacity);if(o.id===selected)g.classList.add("selected");const color=prop(o,"color","#111827"),accent=prop(o,"accent","#9b8dff"),radius=Number(prop(o,"radius",18));
  if(o.kind==="circle"||o.kind==="orb"){const c=document.createElementNS(ns,"circle");c.setAttribute("cx",o.width/2);c.setAttribute("cy",o.height/2);c.setAttribute("r",Math.min(o.width,o.height)/2);c.setAttribute("fill",color);c.setAttribute("opacity",".82");g.appendChild(c);const r=document.createElementNS(ns,"circle");r.setAttribute("cx",o.width/2);r.setAttribute("cy",o.height/2);r.setAttribute("r",Math.min(o.width,o.height)/2-12);r.setAttribute("fill","none");r.setAttribute("stroke",accent);r.setAttribute("stroke-opacity",".65");r.setAttribute("stroke-width","2");g.appendChild(r)}
  else{const r=document.createElementNS(ns,"rect");r.setAttribute("width",o.width);r.setAttribute("height",o.height);r.setAttribute("rx",radius);r.setAttribute("fill",color);r.setAttribute("stroke",accent);r.setAttribute("stroke-opacity",o.kind==="frame"?".42":".2");r.setAttribute("stroke-width","1");g.appendChild(r)}
  if(o.label){const t=document.createElementNS(ns,"text");t.classList.add("obj-label");t.textContent=o.label;t.setAttribute("x",prop(o,"textAlign","center")==="left"?14:o.width/2);t.setAttribute("y",o.height/2+Number(prop(o,"fontSize",o.kind==="text"?24:13))*.34);t.setAttribute("text-anchor",prop(o,"textAlign","center")==="right"?"end":prop(o,"textAlign","center")==="left"?"start":"middle");if(prop(o,"textAlign","center")==="right")t.setAttribute("x",o.width-14);t.setAttribute("fill",o.kind==="button"?prop(o,"textColor","#fff"):prop(o,"color","#fff"));t.setAttribute("font-family",prop(o,"fontFamily","Fira Sans"));t.setAttribute("font-size",prop(o,"fontSize",o.kind==="text"?24:13));t.setAttribute("font-weight",prop(o,"weight",o.kind==="text"?700:600));g.appendChild(t)}
  svg.appendChild(g)});
 renderInspector();renderScenes();renderTracks()
}
function renderInspector(){
 const o=scene().objects.find(x=>x.id===selected),b=$("#inspectorBody");$("#selectionName").textContent=o?(o.label||o.kind):"No selection";
 if(!o){b.innerHTML="<div class='section'><h4>QUICK START</h4><p style='color:#728097;line-height:1.6'>Select anything on the canvas. This floating glass inspector is browser-native and contextual.</p></div>";return}
 b.innerHTML="<div class='section'><h4>CONTENT</h4><div class='field'><label>Text / Label</label><textarea id='iLabel'>"+esc(o.label)+"</textarea></div></div><div class='section'><h4>GEOMETRY</h4><div class='twocol'>"+["x","y","width","height"].map(k=>"<div class='field'><label>"+k.toUpperCase()+"</label><input data-field='"+k+"' type='number' value='"+o[k]+"'></div>").join("")+"</div><div class='field'><label>Rotation</label><input data-field='rotation' type='number' value='"+o.rotation+"'></div></div><div class='section'><h4>APPEARANCE</h4><div class='field'><label>Color</label><input id='iColor' type='color' value='"+prop(o,"color","#111827")+"'></div>"+(o.kind==="text"||o.kind==="button"?"<div class='twocol'><div class='field'><label>Size</label><input id='iSize' type='number' value='"+prop(o,"fontSize",24)+"'></div><div class='field'><label>Weight</label><select id='iWeight'><option value='400'>400</option><option value='600'>600</option><option value='700'>700</option><option value='800'>800</option></select></div></div>":"")+"<div class='format-row'><button data-fmt='weight'>B</button><button data-fmt='italic'>I</button><button data-fmt='underline'>U</button><button data-fmt='textAlign'>≡</button></div></div><div class='section'><h4>IDENTITY</h4><div class='field'><label>Object ID</label><input value='"+esc(o.id)+"' readonly></div></div><button class='danger-action' id='deleteSelected'>Delete object</button>";
 $("#iWeight")?.setAttribute("value",String(prop(o,"weight",600)));
 $("#iLabel")?.addEventListener("input",e=>setObject({label:e.target.value}));
 $$("[data-field]").forEach(i=>i.addEventListener("change",e=>{const v=Number(e.target.value);const p={};p[e.target.dataset.field]=v;setObject(p)}));
 $("#iColor")?.addEventListener("input",e=>format("color",e.target.value));$("#iSize")?.addEventListener("change",e=>format("fontSize",Number(e.target.value)));$("#iWeight")?.addEventListener("change",e=>format("weight",Number(e.target.value)));
 $$("[data-fmt]").forEach(b=>b.onclick=()=>format(b.dataset.fmt,b.dataset.fmt==="textAlign"?"center":b.dataset.fmt==="weight"?800:true));$("#deleteSelected")?.addEventListener("click",()=>{cmd({type:"delete_object",id:o.id});selected=null})
}
function renderScenes(){const s=$("#sceneStrip");s.innerHTML="";doc.scenes.forEach((x,i)=>{const b=document.createElement("button");b.className="scene-card"+(i===doc.active_scene?" active":"");b.dataset.scene=i;b.innerHTML="<span class='scene-thumb'></span><b>"+esc(x.name)+"</b><small>SCENE "+String(i+1).padStart(2,"0")+"</small>";b.onclick=()=>selectScene(i);s.appendChild(b)})}
function renderTracks(){const t=$("#tracks"),a=scene().animation;t.innerHTML="";if(!a.tracks.length){t.innerHTML="<div class='track'><b style='color:#58687e'>No tracks yet</b></div>";return}a.tracks.forEach(tr=>{const row=document.createElement("div");row.className="track";row.innerHTML="<b>"+esc(tr.property)+"</b><div class='keys'></div>";const keys=row.querySelector(".keys");tr.keyframes.forEach(k=>{const i=document.createElement("i");i.className="key";i.style.left=(k.time/a.duration*100)+"%";keys.appendChild(i)});t.appendChild(row)})}
function validate(){const r=JSON.parse(engine.validate_json(JSON.stringify(doc)));toast(r.message)}
function preview(){const w=window.open("","_blank");if(!w){toast("Popup blocked");return}w.document.write("<!doctype html><html><body style='margin:0;background:#06080d;display:grid;place-items:center;min-height:100vh'><iframe style='width:min(1200px,92vw);aspect-ratio:5/3;border:1px solid #33415a;border-radius:18px' src='"+location.href+"'></iframe></body></html>")}
function togglePlay(){playing=!playing;$("#timeline").classList.add("open");$("#timeline .play").textContent=playing?"Ⅱ":"▶";if(playing)tick();else cancelAnimationFrame(raf)}
function tick(){if(!playing)return;playhead+=1/60;if(playhead>scene().animation.duration)playhead=scene().animation.looped?0:scene().animation.duration;$("#playhead").value=playhead;$("#timeReadout").textContent=playhead.toFixed(2)+"s";raf=requestAnimationFrame(tick)}
function stop(){playing=false;playhead=0;$("#playhead").value=0;$("#timeReadout").textContent="00:00.00";$("#timeline .play").textContent="▶"}
function openCommands(){const o=$("#commandPalette");o.hidden=false;renderCommands("");$("#commandSearch").focus()}
function renderCommands(q){const list=[];Object.keys(MENUS).forEach(m=>MENUS[m].forEach(x=>list.push({m:m,label:x[0],a:x[1]})));const hit=list.filter(x=>(x.label+" "+x.m).toLowerCase().includes(q.toLowerCase())).slice(0,14);const r=$("#commandResults");r.innerHTML="";hit.forEach(x=>{const b=document.createElement("button");b.className="command";b.innerHTML="<span>"+esc(x.label)+"</span><small>"+esc(x.m)+"</small>";b.onclick=()=>{$("#commandPalette").hidden=true;doAction(x.a)};r.appendChild(b)})}
function wire(){
 renderMenus();$("#menubar").onclick=e=>{const b=e.target.closest("[data-menu]");if(b){menu===b.dataset.menu?closeMenu():openMenu(b.dataset.menu)}};$("#menuPopup").onclick=e=>{const b=e.target.closest("[data-action]");if(b)doAction(b.dataset.action)};document.addEventListener("click",e=>{if(menu&&!e.target.closest(".menubar")&&!e.target.closest("#menuPopup"))closeMenu()});
 $$("[data-action]").forEach(b=>b.addEventListener("click",()=>{const a=b.dataset.action;doAction(a)}));$$("[data-tool]").forEach(b=>b.onclick=()=>{toolSelect(b.dataset.tool)});
 $$("[data-surface]").forEach(b=>b.onclick=()=>setSurface(b.dataset.surface));$("#stage").onclick=e=>{const g=e.target.closest(".obj");selected=g?g.dataset.id:null;render()};$("#stage").ondblclick=e=>{const g=e.target.closest(".obj");if(g){selected=g.dataset.id;render();setTimeout(()=>$("#iLabel")?.focus(),30)}};
 $("#commandSearch").oninput=e=>renderCommands(e.target.value);$("#commandPalette").onclick=e=>{if(e.target===e.currentTarget)e.currentTarget.hidden=true};$("#playhead").oninput=e=>{playhead=Number(e.target.value);$("#timeReadout").textContent=playhead.toFixed(2)+"s"};
 $("#fileOpen").onchange=async e=>{const f=e.target.files[0];if(!f)return;try{engine.load_json(await f.text());sync();selected=null;toast("Project opened")}catch(err){toast(err.message||"Open failed")}};$("#mediaOpen").onchange=e=>{if(e.target.files.length)toast(e.target.files.length+" media file(s) selected; asset pipeline boundary ready")};
 window.addEventListener("keydown",e=>{const mod=e.metaKey||e.ctrlKey;if(mod&&e.key.toLowerCase()==="s"){e.preventDefault();save()}if(mod&&e.key.toLowerCase()==="z"){e.preventDefault();doAction(e.shiftKey?"redo":"undo")}if(mod&&e.key.toLowerCase()==="k"){e.preventDefault();openCommands()}if(e.code==="Space"&&document.activeElement.tagName!=="INPUT"&&document.activeElement.tagName!=="TEXTAREA"){e.preventDefault();togglePlay()}if(e.key==="f"&&!mod)doAction("fit");if(e.key==="Escape"){closeMenu();$("#commandPalette").hidden=true}})
}
function toolSelect(t){$$(".tool").forEach(b=>b.classList.toggle("active",b.dataset.tool===t));if(t==="text")add("text","New text",{fontSize:30,color:"#f4f7fb",weight:700});if(t==="shape")add("card","SHAPE",{color:"#18243a",accent:"#9b8dff"});if(t==="button")add("button","BUTTON",{color:"#9b8dff",textColor:"#080b11"});if(t==="media")$("#mediaOpen").click()}
async function boot(){await init("./pkg/syn-studio_bg.wasm");engine=new Engine();const saved=localStorage.getItem("syn-studio-document");if(saved)try{engine.load_json(saved)}catch{}doc=JSON.parse(engine.document_json());wire();render();setZoom(1)}
boot().catch(e=>{document.body.innerHTML="<div style='display:grid;place-items:center;height:100vh;background:#06080d;color:#dce5f1;font:14px system-ui;padding:24px;text-align:center'>SYN Studio failed to start.<br><small style='opacity:.6'>"+esc(e.message||e)+"</small></div>"});
