import { createSynDocument, addScene, addObject, addInteraction, serializeSynDocument } from "../src/document.js";
import { createRuntimeState, currentScene, dispatchEvent, renderScene } from "../src/runtime.js";

const state = {
  document: createSynDocument({ title: "SYN Studio Showcase" }),
  sceneIndex: 0,
  selectedObjectId: null,
  drag: null
};

const home = addScene(state.document, { id: "scene-1", name: "SYN / Showcase", background: "#080b10" });
const architecture = addScene(state.document, { id: "scene-2", name: "SYN / Architecture", background: "#0a0e14" });
const interactive = addScene(state.document, { id: "scene-3", name: "SYN / Interactive", background: "#090c12" });
const source = addScene(state.document, { id: "scene-4", name: "SYN / Source", background: "#080b10" });

function text(scene, id, label, x, y, width, height, styles, value=label) {
  return addObject(scene, { id, kind:"text", label, x, y, width, height, props:{text:value}, styles });
}
function card(scene, id, label, x, y, width, height, styles, value=label) {
  return addObject(scene, { id, kind:"shape", label, x, y, width, height, props:{text:value}, styles });
}
function button(scene, id, label, x, y, width, height, styles) {
  return addObject(scene, { id, kind:"button", label, x, y, width, height, props:{text:label}, styles });
}

const muted = "#8f9bad";
const soft = "#c8d0dc";
const white = "#f4f7fb";
const line = "#273140";
const blue = "#8faeff";
const panel = "#10151d";

text(home,"home-kicker","SYN / CREATIVE RUNTIME",72,54,500,24,{fontSize:11,fontWeight:800,color:blue,letterSpacing:3});
text(home,"home-title","SYN",68,92,560,112,{fontSize:96,fontWeight:900,color:white,letterSpacing:-5});
text(home,"home-headline","A new canvas for interactive media.",74,205,620,42,{fontSize:25,fontWeight:650,color:soft});
text(home,"home-copy","Design a document. Add behavior. Ship an experience.\nSYN brings visual authoring, media, navigation and logic into one portable creative object.",76,252,600,76,{fontSize:15,fontWeight:450,color:muted,lineHeight:1.55});
card(home,"home-hero-panel","",720,62,330,285,{background:"linear-gradient(145deg,#182131,#0e131b)",borderColor:"#34445b",borderWidth:1,borderRadius:22,boxShadow:"0 28px 80px #0009"});
text(home,"home-panel-label","DOCUMENT",748,92,160,22,{fontSize:10,fontWeight:800,color:blue,letterSpacing:2});
text(home,"home-panel-value","01",748,120,120,78,{fontSize:64,fontWeight:850,color:white});
text(home,"home-panel-rule","────────────────",748,198,220,18,{fontSize:12,color:"#425069"});
text(home,"home-panel-copy","One file can contain scenes,\nobjects, assets, links, behavior\nand presentation logic.",748,224,245,76,{fontSize:14,color:soft,lineHeight:1.45});
button(home,"home-explore","Explore the document",76,360,190,48,{background:"linear-gradient(180deg,#eef3ff,#b8c7ff)",color:"#09101c",fontSize:13,fontWeight:800,borderRadius:10,boxShadow:"0 12px 30px #0007"});
button(home,"home-source","Open source view",278,360,160,48,{background:panel,color:soft,fontSize:13,fontWeight:750,borderRadius:10,borderColor:line,borderWidth:1});
card(home,"home-stat-1","",72,438,300,84,{background:"#0d1219",borderColor:line,borderWidth:1,borderRadius:12});
text(home,"home-stat-1a","04",92,458,58,34,{fontSize:25,fontWeight:850,color:white});
text(home,"home-stat-1b","scenes in this showcase",155,463,180,22,{fontSize:11,fontWeight:700,color:muted});
card(home,"home-stat-2","",388,438,300,84,{background:"#0d1219",borderColor:line,borderWidth:1,borderRadius:12});
text(home,"home-stat-2a","∞",408,456,58,38,{fontSize:28,fontWeight:850,color:blue});
text(home,"home-stat-2b","creative possibilities",465,463,180,22,{fontSize:11,fontWeight:700,color:muted});
card(home,"home-stat-3","",704,438,346,84,{background:"#0d1219",borderColor:line,borderWidth:1,borderRadius:12});
text(home,"home-stat-3a","01",724,458,58,34,{fontSize:25,fontWeight:850,color:white});
text(home,"home-stat-3b","shared document model",787,463,200,22,{fontSize:11,fontWeight:700,color:muted});
addInteraction(home,{event:{type:"click",target:"home-explore"},actions:[{type:"scene.goto",target:"scene-2"}]});
addInteraction(home,{event:{type:"click",target:"home-source"},actions:[{type:"scene.goto",target:"scene-4"}]});

text(architecture,"arch-kicker","01 / ARCHITECTURE",70,52,300,22,{fontSize:10,fontWeight:800,color:blue,letterSpacing:2.5});
text(architecture,"arch-title","One document.",70,84,650,58,{fontSize:48,fontWeight:850,color:white});
text(architecture,"arch-sub","Everything you see is part of the same inspectable SYN model.",72,145,650,28,{fontSize:16,color:muted});
const cards=[
["Design","VISUAL","Canvas, typography, components and layout.","#16202c"],
["Behavior","LOGIC","Events, actions, navigation and state.","#151d2a"],
["Media","ASSETS","Images, audio, video and embedded resources.","#18202a"],
["Code","SOURCE","Inspectable source with a future sandboxed extension layer.","#171e28"]
];
cards.forEach((c,i)=>{
  const x=72+(i%2)*332,y=205+Math.floor(i/2)*132;
  card(architecture,"arch-card-"+i,"",x,y,300,108,{background:c[3],borderColor:line,borderWidth:1,borderRadius:14,boxShadow:"0 14px 35px #0006"});
  text(architecture,"arch-card-k-"+i,c[0].toUpperCase(),x+20,y+17,120,18,{fontSize:10,fontWeight:850,color:blue,letterSpacing:1.5});
  text(architecture,"arch-card-t-"+i,c[1],x+20,y+40,100,18,{fontSize:15,fontWeight:800,color:white});
  text(architecture,"arch-card-d-"+i,c[2],x+20,y+66,250,34,{fontSize:11,color:muted,lineHeight:1.3});
});
card(architecture,"arch-rail","",760,205,290,240,{background:"linear-gradient(160deg,#111a25,#0c1118)",borderColor:"#34445b",borderWidth:1,borderRadius:16});
text(architecture,"arch-rail-k","THE RULE",785,230,120,18,{fontSize:10,fontWeight:850,color:blue,letterSpacing:2});
text(architecture,"arch-rail-t","A SYN file\nis data,\nnot authority.",785,258,220,112,{fontSize:31,fontWeight:850,color:white,lineHeight:1.08});
text(architecture,"arch-rail-b","Sandboxed runtime • explicit capabilities • declarative behavior",785,382,235,42,{fontSize:10,color:muted,lineHeight:1.35});
button(architecture,"arch-next","Open interactive scene",72,488,190,46,{background:"#dce5ff",color:"#09101c",fontSize:12,fontWeight:800,borderRadius:9});
addInteraction(architecture,{event:{type:"click",target:"arch-next"},actions:[{type:"scene.goto",target:"scene-3"}]});

text(interactive,"int-kicker","02 / INTERACTION",70,52,300,22,{fontSize:10,fontWeight:800,color:blue,letterSpacing:2.5});
text(interactive,"int-title","Behavior is part of the composition.",70,84,720,58,{fontSize:43,fontWeight:850,color:white});
text(interactive,"int-copy","This button is not a mockup. It is wired to the same declarative runtime that renders the document.",72,147,690,46,{fontSize:15,color:muted,lineHeight:1.45});
card(interactive,"int-console","",72,220,670,245,{background:"#0b1017",borderColor:"#2c3747",borderWidth:1,borderRadius:16,boxShadow:"0 20px 55px #0008"});
text(interactive,"int-console-k","EVENT GRAPH",96,244,140,18,{fontSize:10,fontWeight:850,color:blue,letterSpacing:2});
text(interactive,"int-line-1","WHEN",96,284,70,24,{fontSize:12,fontWeight:850,color:"#d9a7ff"});
text(interactive,"int-line-2","Button / Continue",170,284,180,24,{fontSize:12,fontWeight:700,color:soft});
text(interactive,"int-line-3","CLICKED",355,284,90,24,{fontSize:12,fontWeight:850,color:"#9ce5c0"});
text(interactive,"int-arrow","→",450,282,30,26,{fontSize:16,fontWeight:900,color:"#5c6b80"});
text(interactive,"int-line-4","SET TEXT",490,284,110,24,{fontSize:12,fontWeight:850,color:blue});
text(interactive,"int-line-5","StoryCard",96,329,130,24,{fontSize:12,fontWeight:700,color:soft});
text(interactive,"int-line-6","Runtime updated.",230,329,230,24,{fontSize:12,color:"#c9d3e3"});
card(interactive,"int-live","LIVE STATE",785,220,265,245,{background:"linear-gradient(160deg,#162234,#0c121a)",borderColor:"#34445b",borderWidth:1,borderRadius:16});
text(interactive,"int-live-k","LIVE STATE",810,246,120,18,{fontSize:10,fontWeight:850,color:"#9ce5c0",letterSpacing:2});
text(interactive,"int-live-v","READY",810,276,180,40,{fontSize:28,fontWeight:850,color:white});
text(interactive,"int-live-copy","Click Continue to mutate the card below.",810,327,190,45,{fontSize:12,color:muted,lineHeight:1.4});
card(interactive,"int-story","",72,490,670,82,{background:"#111821",borderColor:line,borderWidth:1,borderRadius:12});
text(interactive,"int-story-text","The story is waiting for an event.",94,514,610,28,{fontSize:15,fontWeight:650,color:soft});
button(interactive,"int-continue","Continue",785,500,145,46,{background:"linear-gradient(180deg,#eef3ff,#b8c7ff)",color:"#09101c",fontSize:12,fontWeight:800,borderRadius:9});
button(interactive,"int-back","Back to architecture",940,500,110,46,{background:"#10151d",color:soft,fontSize:11,fontWeight:750,borderRadius:9,borderColor:line,borderWidth:1});
addInteraction(interactive,{event:{type:"click",target:"int-continue"},actions:[{type:"object.setText",target:"int-story-text",value:"The story changed at runtime. No page reload. No custom DOM hack."}]});
addInteraction(interactive,{event:{type:"click",target:"int-back"},actions:[{type:"scene.goto",target:"scene-2"}]});

text(source,"src-code-1",'{  "syn": "0.1",',98,232,620,22,{fontFamily:"ui-monospace, SFMono-Regular, Consolas, monospace",fontSize:12,color:"#c8d2e3"});
text(source,"src-code-2",'  "type": "document",',98,258,620,22,{fontFamily:"ui-monospace, SFMono-Regular, Consolas, monospace",fontSize:12,color:"#c8d2e3"});
text(source,"src-copy","Visual authoring and source inspection target the same document model.",72,145,650,28,{fontSize:16,color:muted});
text(source,"src-code-4",'    { "id": "scene-3",',98,310,620,22,{fontFamily:"ui-monospace, SFMono-Regular, Consolas, monospace",fontSize:12,color:"#c8d2e3"});
text(source,"src-code-5",'      "objects": [ ... ],',98,336,620,22,{fontFamily:"ui-monospace, SFMono-Regular, Consolas, monospace",fontSize:12,color:"#9ce5c0"});
text(source,"src-code-2","  "type": "document",",98,258,620,22,{fontFamily:"ui-monospace, SFMono-Regular, Consolas, monospace",fontSize:12,color:"#c8d2e3"});
text(source,"src-code-3",'  "scenes": [',98,284,620,22,{fontFamily:"ui-monospace, SFMono-Regular, Consolas, monospace",fontSize:12,color:"#8faeff"});
text(source,"src-code-4","    { "id": "scene-3",",98,310,620,22,{fontFamily:"ui-monospace, SFMono-Regular, Consolas, monospace",fontSize:12,color:"#c8d2e3"});
text(source,"src-code-5","      "objects": [ ... ],",98,336,620,22,{fontFamily:"ui-monospace, SFMono-Regular, Consolas, monospace",fontSize:12,color:"#9ce5c0"});
text(source,"src-code-6",'      "interactions": [ ... ]',98,362,620,22,{fontFamily:"ui-monospace, SFMono-Regular, Consolas, monospace",fontSize:12,color:"#d9a7ff"});
text(source,"src-code-7","    }",98,388,620,22,{fontFamily:"ui-monospace, SFMono-Regular, Consolas, monospace",fontSize:12,color:"#c8d2e3"});
text(source,"src-code-8","  ]",98,414,620,22,{fontFamily:"ui-monospace, SFMono-Regular, Consolas, monospace",fontSize:12,color:"#8faeff"});
text(source,"src-code-9","}",98,440,620,22,{fontFamily:"ui-monospace, SFMono-Regular, Consolas, monospace",fontSize:12,color:"#c8d2e3"});
card(source,"src-note","",860,205,190,165,{background:"linear-gradient(155deg,#141b25,#0d1219)",borderColor:"#34445b",borderWidth:1,borderRadius:14});
text(source,"src-note-k","INSPECTABLE",884,231,130,18,{fontSize:10,fontWeight:850,color:blue,letterSpacing:1.5});
text(source,"src-note-v","Readable.\nPortable.\nDeterministic.",884,264,140,80,{fontSize:21,fontWeight:800,color:white,lineHeight:1.2});
text(source,"src-note-b","No arbitrary code execution in the core runtime.",884,345,140,42,{fontSize:10,color:muted,lineHeight:1.35});
button(source,"src-home","Return home",72,570,145,46,{background:"#10151d",color:soft,fontSize:12,fontWeight:750,borderRadius:9,borderColor:line,borderWidth:1});
button(source,"src-arch","View architecture",230,570,165,46,{background:"#dce5ff",color:"#09101c",fontSize:12,fontWeight:800,borderRadius:9});
addInteraction(source,{event:{type:"click",target:"src-home"},actions:[{type:"scene.goto",target:"scene-1"}]});
addInteraction(source,{event:{type:"click",target:"src-arch"},actions:[{type:"scene.goto",target:"scene-2"}]});

state.selectedObjectId = null;

const MENU_DATA = {
File:["New SYN","New from Template","Open","Open Recent","Save","Save As","Save a Copy","Save as Template","Auto Save","Version History","Restore Version","Duplicate Project","Rename Project","Project Information","Import","Export","Share","Publish","Project Settings","Document Settings","Permissions","Close"],
Edit:["Undo","Redo","History","Cut","Copy","Paste","Paste in Place","Duplicate","Delete","Select All","Select None","Select Similar","Find","Find & Replace","Rename","Lock","Unlock","Group","Ungroup","Preferences"],
View:["Zoom In","Zoom Out","Actual Size","Fit Canvas","Fit Selection","Fullscreen","Presentation Mode","Grid","Guides","Snap","Rulers","Safe Areas","Show Layers","Show Assets","Show Timeline","Show Interactions","Show Code","Show Console","Reset Workspace"],
Insert:["Text","Rich Text","Heading","Paragraph","Link","Button","Shape","Line","Arrow","Group","Component","Image","SVG","GIF","Audio","Video","Gallery","Slideshow","Embed","Web Content","Form","Input","Checkbox","Toggle","Dropdown","Menu","Icon","QR Code","Map","Chart","Table","Timer","Scene","Hotspot","Popup","Modal","Tooltip","Animation","Code Block"],
Format:["Font","Font Size","Weight","Style","Text Color","Gradient","Highlight","Alignment","Justification","Letter Spacing","Line Height","Text Case","Decoration","Outline","Shadow","Glow","Fill","Stroke","Stroke Width","Corners","Opacity","Blend Mode","Transform","Position","Dimensions"],
Arrange:["Align Left","Align Center","Align Right","Align Top","Align Middle","Align Bottom","Distribute Horizontally","Distribute Vertically","Match Width","Match Height","Match Size","Equal Spacing","Center on Canvas","Group","Ungroup","Lock","Hide","Duplicate","Flip","Rotate","Reset Transform","Smart Guides","Snap"],
Object:["New Object","Duplicate","Clone","Instance","Convert to Group","Convert to Component","Convert to Path","Convert to Shape","Convert to Text","Object Properties","Object Metadata","Object ID","Parent","Children","Dependencies","References","Bindings","States","Variables","Events","Constraints","Visibility","Delete"],
Project:["Dashboard","Structure","Files","Scenes","Assets","Components","Templates","Libraries","Dependencies","Packages","Development","Testing","Production","Variables","Secrets","Runtime Configuration","Build Configuration","Publishing Configuration","Metadata","Authors","Contributors","License","Diagnostics"],
Scene:["New Scene","Duplicate Scene","Delete Scene","Rename Scene","Scene Properties","Background","Dimensions","Aspect Ratio","Transition","Entry Animation","Exit Animation","Scene Link","Navigation","Scene Order","Thumbnail","Master Objects","Scene Variables","Scene Graph","Layers","Camera"],
Timeline:["New Timeline","Layers","Tracks","Keyframes","Blank Keyframe","Frame","Insert Frame","Delete Frame","Copy Frame","Paste Frame","Tween","Motion Tween","Shape Tween","Morph","Markers","Timecode","Playback","Loop","In Point","Out Point","Time Stretch","Speed","Onion Skin","Curve Editor","Dope Sheet"],
Animation:["New Animation","Keyframes","Position","Scale","Rotation","Opacity","Color","Size","Blur","Shadow","Transform","Easing","Linear","Ease In","Ease Out","Ease In Out","Spring","Bounce","Motion Path","Loop","Ping Pong","Delay","Duration","Preview"],
Media:["Import Media","Media Library","Asset Manager","Replace Asset","Relink Asset","Embed Asset","Extract Asset","Compress","Optimize","Crop","Mask","Fit","Playback","Volume","Loop","Autoplay","Captions","Poster Frame","Metadata","Dependencies"],
Audio:["Import Audio","Record Audio","Waveform","Trim","Split","Fade In","Fade Out","Volume","Pan","Speed","Pitch","Loop","Autoplay","Play","Pause","Stop","Seek","Synchronize","Audio Timeline","Sound Effects","Voiceover","Audio Ducking","Visualization"],
Video:["Import Video","Camera","Video Library","Trim","Split","Crop","Resize","Playback","Loop","Autoplay","Controls","Volume","Speed","Poster Frame","Captions","Subtitles","Timeline","Transitions","Masks","Filters","Color Adjustment","Chroma Key"],
Typography:["Font Family","Font Size","Weight","Style","Color","Gradient","Alignment","Justification","Line Height","Letter Spacing","Word Spacing","Text Case","Decoration","Outline","Shadow","Columns","Wrapping","Vertical Alignment","Language","Spell Check","Text on Path","Rich Text","Markdown","Dynamic Text"],
Components:["Create Component","Edit Component","Component Library","Local Components","Shared Components","Variants","States","Properties","Inputs","Outputs","Events","Slots","Overrides","Instances","Detach Instance","Swap Instance","Nested Components","Dependencies","Documentation","Publish Component"],
Symbols:["Create Symbol","Graphic Symbol","Movie Symbol","Interactive Symbol","Button Symbol","Edit Symbol","Symbol Library","Instance","Properties","Timeline","States","Convert to Component"],
Layout:["Freeform","Absolute","Relative","Flex","Grid","Stack","Flow","Auto Layout","Constraints","Anchoring","Alignment","Distribution","Padding","Margin","Gap","Min Width","Max Width","Min Height","Max Height","Overflow","Scroll","Z Index","Layout Inspector"],
Responsive:["Desktop","Tablet","Mobile","Landscape","Portrait","Custom Breakpoint","Responsive Layout","Fluid Width","Fixed Width","Constraints","Auto Layout","Hide on Mobile","Hide on Desktop","Reposition","Resize","Responsive Typography"],
Interaction:["Events","Click","Double Click","Hover","Pointer Down","Pointer Up","Drag","Drop","Touch","Swipe","Scroll","Key Down","Key Up","Focus","Blur","Load","Scene Enter","Scene Exit","Timer","Variable Changed","Data Loaded","Conditions","If","Else","And","Or","Not","Actions","Navigate","Animate","Set Property","Set Variable","Show","Hide","Toggle","Create","Destroy","Play Media","Pause Media","Call Function","Call API","Dispatch Event"],
Navigation:["Link to Scene","Link to Object","Link to SYN","Link to URL","Open New Window","Open Same Window","Back","Forward","Home","Scene Menu","Breadcrumbs","Tabs","Previous Scene","Next Scene","Jump to Scene","Deep Link","Generate Share Link","Generate QR Code","Navigation History"],
Logic:["Event","Condition","Action","Sequence","Branch","Loop","Repeat","Wait","Parallel","Variable","Function","State","Trigger","Signal","Broadcast","Listen","Logic Graph"],
State:["Object State","Scene State","Global State","Create State","Duplicate State","State Variables","Transitions","Conditions","Events","Initial State","Persistent State","Reset State"],
Signals:["Create Signal","Dispatch Signal","Listen","Global Signals","Scene Signals","Component Signals","Event Bus","Signal Inspector","Signal History"],
Variables:["Create Variable","Global Variables","Scene Variables","Object Variables","Component Variables","Constants","Boolean","Number","String","Array","Object","Date","Color","Vector","Reference","Environment Variable","Watch","Bind","Inspector"],
Data:["Variables","Collections","Data Sources","Data Binding","Data Mapping","Transform","Filter","Sort","Search","Validation","JSON","CSV","XML","Local Storage","Session Storage","Cache","State Inspector"],
Database:["Connections","Add Connection","REST Database","SQL Database","Query","Insert","Update","Delete","Transactions","Schemas","Tables","Views","Authentication","Permissions","Secrets","Data Models","Migrations","Logs","Connection Test"],
API:["REST","GraphQL","WebSocket","HTTP Request","GET","POST","PUT","PATCH","DELETE","Headers","Parameters","Body","Authentication","OAuth","API Keys","Responses","Error Handling","Mock Response","API Explorer","Request Inspector"],
Forms:["Text Input","Password","Email","Number","Date","Time","Checkbox","Radio","Toggle","Slider","Dropdown","Multi Select","File Upload","Search","Submit","Reset","Validation","Required Fields","Error Messages","Success Messages","Form Actions","Data Binding"],
Web:["Hyperlink","URL","Embed","IFrame","Web Component","HTML","CSS","Metadata","SEO","Open Graph","Favicon","Custom Headers","External Resources","Web Fonts","Web APIs","Responsive Preview","Mobile Preview","Desktop Preview"],
Code:["Code Editor","JavaScript","Modules","Functions","Classes","Variables","Events","Imports","Exports","Libraries","Packages","Snippets","Type Definitions","Runtime API","Scene API","Data API","Media API","Animation API","Network API","Storage API","Capability API","Console","Formatter","Linter","Type Checker","Documentation"],
Debug:["Start Debugging","Run","Pause","Stop","Restart","Step Over","Step Into","Step Out","Breakpoints","Conditional Breakpoints","Watch","Variables","Call Stack","Events","Network","Console","Errors","Warnings","Runtime State","Scene State","Object State","Performance","Memory","Rendering","Debug Overlay"],
AI:["AI Assistant","Ask About Project","Generate Text","Rewrite Text","Summarize","Translate","Generate Image","Edit Image","Remove Background","Generate Video","Generate Audio","Generate Music","Generate Layout","Generate Scene","Generate Animation","Generate Component","Generate Code","Explain Code","Fix Code","Analyze Document","Organize Document","Generate Accessibility","Generate Metadata","Optimize Media","Create From Prompt","AI History","AI Permissions"],
Assets:["Asset Library","Import","Export","Images","Audio","Video","Fonts","SVG","Icons","3D","Documents","External Assets","Embedded Assets","Metadata","Dependencies","Replace","Relink","Optimize","Compress","Convert","Find Unused"],
Effects:["Shadow","Inner Shadow","Glow","Outer Glow","Blur","Motion Blur","Gaussian Blur","Color Adjust","Brightness","Contrast","Saturation","Hue","Grayscale","Sepia","Invert","Opacity","Blend Modes","Distortion","Noise","Pixelate","Mask","Clipping","Gradient","Pattern","Reflection"],
Accessibility:["Inspector","Alt Text","Semantic Roles","Keyboard Navigation","Focus Order","Screen Reader Labels","Captions","Transcripts","Color Contrast","Reduced Motion","Text Scaling","Accessible Links","Accessible Forms"],
Localization:["Languages","Add Language","Translation Strings","String Tables","Text Variants","Locale","Number Formatting","Date Formatting","Currency","Right to Left","Import","Export","Missing Translation Report"],
Security:["Permissions","Capabilities","Network Access","Storage Access","Camera","Microphone","Location","External Resources","Embedded Content","Script Permissions","API Permissions","Database Permissions","Secrets","Sandbox","Trust","Signature","Verify Signature","Security Audit","Security Report"],
Performance:["Monitor","FPS","CPU","Memory","GPU","Rendering","Network","Asset Size","Scene Complexity","Animation Cost","Media Cost","Load Time","Runtime Profiling","Bottlenecks","Optimization Suggestions","Performance Report"],
Version:["Save Version","History","Compare","Restore","Branch","Merge","Snapshot","Release","Release Notes","Changelog","Tags","Draft","Stable","Experimental"],
Collaboration:["Share","Invite","Comments","Mentions","Suggestions","Review Mode","Presentation Mode","Live Editing","Cursor Presence","Change History","Approvals","Resolve Comments","Permissions"],
Build:["Build Project","Build SYN","Validate","Optimize","Minify","Compress","Bundle Assets","Embed Assets","Externalize Assets","Manifest","Metadata","Dependencies","Production","Development","Debug","Test","Browser","Desktop","Mobile","Embedded Runtime","Output"],
Package:["Package SYN","Embedded Assets","External Assets","Dependencies","Fonts","Components","Libraries","Metadata","Manifest","Integrity","Signature","Compression","Optimization","Inspect","Validate","Report"],
Publish:["Export SYN","Publish SYN","Publish Web","Static HTML","PDF","PNG","JPEG","SVG","GIF","Video","Presentation","Share Link","QR Code","Embed Code","Version","Release","Draft","Preview"],
Tools:["Command Palette","Asset Inspector","Document Inspector","Object Inspector","Scene Inspector","Runtime Inspector","Event Inspector","Data Inspector","API Inspector","Network Inspector","Storage Inspector","Dependency Graph","Object Graph","Scene Graph","Render Tree","Accessibility Tree","Source Viewer","Manifest Viewer","SYN Schema Viewer","Diff","Screenshot","Screen Recording","Import Wizard","Export Wizard","Migration Tools"],
Window:["Workspace","Inspector","Layers","Scenes","Assets","Interactions","Timeline","Animation","Audio","Video","Code","Console","Output","History","Versions","Templates","Components","Project","Data","Network","Accessibility","Security","Performance","AI"],
Help:["Getting Started","Tutorials","Keyboard Shortcuts","SYN Format Documentation","Runtime Documentation","JavaScript API","Capability API","Examples","Templates","Troubleshooting","Developer Documentation","About SYN","About SYN Studio","Experimental Features"]
};
function escMenu(v){return String(v).replace(/[&<>"']/g,c=>({"&":"&amp;","<":"&lt;",">":"&gt;",'"':"&quot;","'":"&#39;"}[c]));}
function buildApplicationMenus(){
  const nav=document.createElement("nav");nav.className="menu-bar";
  nav.innerHTML=Object.entries(MENU_DATA).map(([name,items])=>'<div class="menu"><button class="menu-trigger">'+escMenu(name)+'</button><div class="menu-panel">'+items.map(item=>'<button class="menu-item" data-command="'+escMenu(item)+'">'+escMenu(item)+'</button>').join("")+'</div></div>').join("");
  const actions=document.querySelector(".top-actions");actions.parentElement.insertBefore(nav,actions);
  nav.querySelectorAll(".menu-trigger").forEach(b=>b.addEventListener("click",e=>{
    const m=e.currentTarget.parentElement;
    nav.querySelectorAll(".menu.open").forEach(x=>{if(x!==m)x.classList.remove("open")});
    m.classList.toggle("open");
    const panel=m.querySelector(".menu-panel");
    if(m.classList.contains("open")){
      const r=b.getBoundingClientRect();
      const width=Math.min(360,Math.max(250,window.innerWidth-20));
      const left=Math.min(Math.max(10,r.left),Math.max(10,window.innerWidth-width-10));
      panel.style.width=width+"px"; panel.style.left=left+"px"; panel.style.top=Math.min(window.innerHeight-20,r.bottom+4)+"px";
    } else { panel.style.left=""; panel.style.top=""; }
  }));
  window.addEventListener("resize",()=>nav.querySelectorAll(".menu.open").forEach(m=>{
    const b=m.querySelector(".menu-trigger"),panel=m.querySelector(".menu-panel"); if(!b||!panel)return;
    const r=b.getBoundingClientRect(),width=panel.offsetWidth||280;
    panel.style.left=Math.min(Math.max(10,r.left),Math.max(10,window.innerWidth-width-10))+"px";
    panel.style.top=Math.min(window.innerHeight-20,r.bottom+4)+"px";
  }));
  nav.querySelectorAll(".menu-item").forEach(b=>b.addEventListener("click",()=>runMenuCommand(b.dataset.command)));
}
function runMenuCommand(c){
  document.querySelectorAll(".menu.open").forEach(x=>x.classList.remove("open"));
  const handlers={"New SYN":()=>document.querySelector("#newDocument").click(),"Open":()=>document.querySelector("#openSyn").click(),"Save":()=>document.querySelector("#export").click(),"Export":()=>document.querySelector("#export").click(),"Delete":()=>document.querySelector("#deleteObject").click(),"New Scene":()=>document.querySelector("#newScene").click(),"Text":()=>addCanvasObject("text"),"Image":()=>document.querySelector("#mediaInput").click(),"Button":()=>addCanvasObject("button"),"Shape":()=>addCanvasObject("shape"),"Preview":()=>document.querySelector("#preview").click(),"Code Editor":openCodeWorkspace,"Command Palette":openCodePalette,"Fullscreen":()=>document.documentElement.requestFullscreen?.(),"Grid":()=>stage.classList.toggle("no-grid"),"Fit Canvas":()=>showToast("Canvas fit command ready.")};
  if(handlers[c])handlers[c]();else showToast(c+" is part of the SYN capability surface and is not enabled in this prototype yet.");
}
function showToast(message){let t=document.querySelector(".toast");if(!t){t=document.createElement("div");t.className="toast";document.body.appendChild(t)}t.textContent=message;t.classList.add("show");clearTimeout(showToast.timer);showToast.timer=setTimeout(()=>t.classList.remove("show"),2200)}
function openCodePalette(){let p=document.querySelector("#syn-command-palette");if(!p){p=document.createElement("div");p.id="syn-command-palette";p.className="modal";p.innerHTML='<div class="palette"><input id="command-search" placeholder="Search commands, tools, scenes..."><div id="command-results"></div></div>';document.body.appendChild(p);p.onclick=e=>{if(e.target===p)p.hidden=true};p.querySelector("input").addEventListener("input",renderCommandResults)}p.hidden=false;p.querySelector("input").focus();renderCommandResults()}
function renderCommandResults(){const p=document.querySelector("#syn-command-palette"),q=p.querySelector("input").value.toLowerCase(),rows=Object.entries(MENU_DATA).flatMap(([menu,items])=>items.map(name=>({menu,name}))).filter(x=>!q||(x.name+" "+x.menu).toLowerCase().includes(q)).slice(0,50);p.querySelector("#command-results").innerHTML=rows.map(x=>'<button class="command-row" data-command="'+escMenu(x.name)+'"><span>'+escMenu(x.name)+'</span><small>'+escMenu(x.menu)+'</small></button>').join("");p.querySelectorAll(".command-row").forEach(b=>b.addEventListener("click",()=>{p.hidden=true;runMenuCommand(b.dataset.command)}))}
function openCodeWorkspace(){
  let panel=document.querySelector("#syn-code-workspace");
  if(!panel){panel=document.createElement("section");panel.id="syn-code-workspace";panel.className="code-workspace";panel.innerHTML='<div class="code-head"><strong>SYN SOURCE</strong><span>Live document model</span></div><textarea id="syn-source" spellcheck="false"></textarea><div class="code-foot"><button id="apply-syn-source" class="primary">Apply Source</button><span>Visual editing and source editing target the same SYN document. JavaScript remains sandboxed and capability-gated.</span></div>';document.querySelector(".timeline").parentElement.appendChild(panel);panel.querySelector("#apply-syn-source").addEventListener("click",()=>{try{const parsed=JSON.parse(panel.querySelector("#syn-source").value);createRuntimeState(parsed);state.document=parsed;state.sceneIndex=0;state.selectedObjectId=null;render();showToast("Source applied to the live document.")}catch(e){showToast(e.message)}})}
  panel.hidden=false;panel.querySelector("#syn-source").value=serializeSynDocument(state.document);panel.scrollIntoView({behavior:"smooth"});
}
function addStyleInspector(){
  const object=selected();if(!object)return;
  let section=inspector.querySelector(".style-section");
  if(section)section.remove();
  section=document.createElement("div");section.className="style-section interaction";
  section.innerHTML='<div class="eyebrow">APPEARANCE</div><div class="field-row"><label class="field">Font<select id="synFont"><option>Inter</option><option>Georgia</option><option>Arial</option><option>Courier New</option><option>Trebuchet MS</option><option>Times New Roman</option><option>system-ui</option></select></label><label class="field">Size<input id="synSize" type="number" min="8" value="'+(object.styles?.fontSize||16)+'"></label></div><div class="field-row"><label class="field">Text color<input id="synColor" type="color" value="'+(/^#[0-9a-f]{6}$/i.test(object.styles?.color||"")?object.styles.color:"#eef1f6")+'"></label><label class="field">Radius<input id="synRadius" type="number" min="0" value="'+(object.styles?.borderRadius||0)+'"></label></div><div class="field-row"><label class="field">Background<input id="synBackground" value="'+escMenu(object.styles?.background||"")+'"></label><label class="field">Weight<select id="synWeight"><option value="400">400</option><option value="600">600</option><option value="700">700</option><option value="800">800</option><option value="900">900</option></select></label></div>';
  inspector.appendChild(section);section.querySelector("#synFont").value=object.styles?.fontFamily||"Inter";section.querySelector("#synWeight").value=object.styles?.fontWeight||400;
  ["synFont","synSize","synColor","synRadius","synBackground","synWeight"].forEach(id=>section.querySelector("#"+id).addEventListener("input",()=>{object.styles={...object.styles,fontFamily:section.querySelector("#synFont").value,fontSize:Number(section.querySelector("#synSize").value)||16,color:section.querySelector("#synColor").value,borderRadius:Number(section.querySelector("#synRadius").value)||0,background:section.querySelector("#synBackground").value,fontWeight:Number(section.querySelector("#synWeight").value)||400};render(false)}));
}


const stage = document.querySelector("#stage");
const emptyState = document.querySelector("#emptyState");
const inspector = document.querySelector("#inspectorContent");
const interactionList = document.querySelector("#interactionList");
const sceneLabel = document.querySelector("#sceneLabel");
const deleteButton = document.querySelector("#deleteObject");

function scene() { return state.document.scenes[state.sceneIndex]; }
function selected() { return scene().objects.find(item => item.id === state.selectedObjectId) ?? null; }

function addCanvasObject(kind) {
  if (kind === "media") {
    document.querySelector("#mediaInput").click();
    return;
  }
  const labels = { text: "Text", media: "Media", shape: "Shape", button: "Button" };
  const count = scene().objects.length;
  const object = addObject(scene(), {
    kind,
    label: labels[kind] ?? "Object",
    x: 60 + (count % 5) * 34,
    y: 60 + (count % 5) * 34,
    width: kind === "text" ? 220 : 160,
    height: kind === "text" ? 64 : 56,
    props: kind === "text" ? { text: "Double-click to edit" } : {}
  });
  state.selectedObjectId = object.id;
  render();
}

function renderDocumentInspector() {
  inspector.innerHTML = `
    <div class="eyebrow">DOCUMENT</div>
    <label class="field">Title<input id="documentTitle" value="${escapeHtml(state.document.meta.title)}"></label>
    <label class="field">Scene name<input id="sceneNameInput" value="${escapeHtml(scene().name)}"></label>
    <div class="interaction"><div>Scenes</div><code>${state.document.scenes.length}</code></div>
    <div class="interaction"><div>Objects</div><code>${scene().objects.length}</code></div>
  `;
  inspector.querySelector("#documentTitle").addEventListener("input", event => {
    state.document.meta.title = event.target.value || "Untitled SYN";
  });
  inspector.querySelector("#sceneNameInput").addEventListener("input", event => {
    scene().name = event.target.value || "Scene";
    sceneLabel.textContent = scene().name;
  });
}

function selectObject(id) {
  state.selectedObjectId = id;
  document.querySelectorAll(".syn-object").forEach(el => el.classList.toggle("selected", el.dataset.id === id));
  deleteButton.disabled = !id;
  const object = selected();
  if (!object) { renderDocumentInspector(); return; }
  inspector.innerHTML = `
    <div class="eyebrow">OBJECT</div>
    <label class="field">Label<input id="objectLabel" value="${escapeHtml(object.label)}"></label>
    <div class="field-row"><label class="field">X<input id="objectX" type="number" value="${object.x}"></label><label class="field">Y<input id="objectY" type="number" value="${object.y}"></label></div>
    <div class="field-row"><label class="field">Width<input id="objectW" type="number" min="20" value="${object.width}"></label><label class="field">Height<input id="objectH" type="number" min="20" value="${object.height}"></label></div>
    <label class="field">Text<textarea id="objectText" rows="3">${escapeHtml(object.props?.text ?? "")}</textarea></label>
    <div class="field-row"><label class="field">Font<select id="objectFont"><option>Inter</option><option>Georgia</option><option>Arial</option><option>Courier New</option><option>Trebuchet MS</option><option>Times New Roman</option><option>system-ui</option></select></label><label class="field">Size<input id="objectFontSize" type="number" min="8" value="${object.styles?.fontSize ?? 16}"></label></div>
    <div class="field-row"><label class="field">Color<input id="objectColor" type="color" value="${/^#[0-9a-f]{6}$/i.test(object.styles?.color ?? "") ? object.styles.color : "#eef1f6"}"></label><label class="field">Radius<input id="objectRadius" type="number" min="0" value="${object.styles?.borderRadius ?? 0}"></label></div>
    <label class="field">Background<input id="objectBackground" value="${escapeHtml(object.styles?.background ?? "")}"></label>
    <div class="interaction"><div>Kind</div><code>${escapeHtml(object.kind)}</code></div>
  `;
  const font=inspector.querySelector("#objectFont"); font.value=object.styles?.fontFamily || "Inter";
  for (const id of ["objectLabel","objectX","objectY","objectW","objectH","objectText","objectFont","objectFontSize","objectColor","objectRadius","objectBackground"]) inspector.querySelector("#"+id).addEventListener("input", updateSelectedObject);
}
function updateSelectedObject() {
  const object = selected(); if (!object) return;
  object.label = inspector.querySelector("#objectLabel").value;
  object.x = Math.max(0, Number(inspector.querySelector("#objectX").value) || 0);
  object.y = Math.max(0, Number(inspector.querySelector("#objectY").value) || 0);
  object.width = Math.max(20, Number(inspector.querySelector("#objectW").value) || 20);
  object.height = Math.max(20, Number(inspector.querySelector("#objectH").value) || 20);
  object.props = { ...object.props, text: inspector.querySelector("#objectText").value };
  object.styles = { ...object.styles, fontFamily: inspector.querySelector("#objectFont").value, fontSize: Number(inspector.querySelector("#objectFontSize").value) || 16, color: inspector.querySelector("#objectColor").value, borderRadius: Number(inspector.querySelector("#objectRadius").value) || 0, background: inspector.querySelector("#objectBackground").value };
  render(false); selectObject(object.id);
}
function deleteSelectedObject() {
  if (!state.selectedObjectId) return;
  const id = state.selectedObjectId;
  scene().objects = scene().objects.filter(object => object.id !== id);
  scene().interactions = scene().interactions.filter(item => item.event.target !== id && !item.actions.some(action => action.target === id));
  state.selectedObjectId = null;
  render();
}

function addNewScene() {
  const next = state.document.scenes.length + 1;
  addScene(state.document, { name: `Scene ${next}` });
  state.sceneIndex = state.document.scenes.length - 1;
  state.selectedObjectId = null;
  render();
}

function changeScene(delta) {
  state.sceneIndex = Math.max(0, Math.min(state.document.scenes.length - 1, state.sceneIndex + delta));
  state.selectedObjectId = null;
  render();
}

function addNewInteraction() {
  const button = scene().objects.find(item => item.kind === "button");
  if (!button) return alert("Add a Button first.");
  const target = state.document.scenes[state.sceneIndex + 1];
  addInteraction(scene(), {
    event: { type: "click", target: button.id },
    actions: [{ type: target ? "scene.goto" : "scene.next", ...(target ? { target: target.id } : {}) }]
  });
  renderInteractions();
}

function removeInteraction(id) {
  scene().interactions = scene().interactions.filter(item => item.id !== id);
  renderInteractions();
}

function renderInteractions() {
  if (!scene().interactions.length) {
    interactionList.innerHTML = '<div class="interaction empty">No interactions yet. Add a Button and give it something to do.</div>';
    return;
  }
  const objectOptions = scene().objects.map(object => `<option value="${escapeHtml(object.id)}">${escapeHtml(object.label)}</option>`).join("");
  const sceneOptions = state.document.scenes.map(target => `<option value="${escapeHtml(target.id)}">${escapeHtml(target.name)}</option>`).join("");
  interactionList.innerHTML = scene().interactions.map(item => {
    const action = item.actions[0] ?? { type: "scene.next" };
    const source = scene().objects.find(object => object.id === item.event.target);
    const targetOptions = action.type === "scene.goto" ? sceneOptions : objectOptions;
    return `<div class="interaction">
      <div class="interaction-rule"><strong>WHEN</strong> <span class="target-chip">${escapeHtml(source?.label ?? item.event.target)}</span> <code>clicked</code></div>
      <span class="flow">→</span>
      <div class="interaction-rule">
        <strong>THEN</strong>
        <select class="interaction-action" data-id="${item.id}">
          <option value="scene.next" ${action.type === "scene.next" ? "selected" : ""}>Next scene</option>
          <option value="scene.goto" ${action.type === "scene.goto" ? "selected" : ""}>Go to scene</option>
          <option value="object.show" ${action.type === "object.show" ? "selected" : ""}>Show object</option>
          <option value="object.hide" ${action.type === "object.hide" ? "selected" : ""}>Hide object</option>
          <option value="object.setText" ${action.type === "object.setText" ? "selected" : ""}>Set object text</option>
        </select>
        ${action.type !== "scene.next" ? `<select class="interaction-target" data-id="${item.id}">${targetOptions}</select>` : ""}
        ${action.type === "object.setText" ? `<input class="interaction-value" data-id="${item.id}" value="${escapeHtml(action.value ?? "")}" placeholder="Text">` : ""}
      </div>
      <button class="remove-interaction" data-id="${item.id}" aria-label="Remove interaction">×</button>
    </div>`;
  }).join("");

  scene().interactions.forEach(item => {
    const actionSelect = interactionList.querySelector(`.interaction-action[data-id="${item.id}"]`);
    if (!actionSelect) return;
    actionSelect.addEventListener("change", () => {
      const action = item.actions[0] ?? { type: "scene.next" };
      action.type = actionSelect.value;
      delete action.target;
      delete action.value;
      if (action.type === "scene.goto") action.target = state.document.scenes[state.sceneIndex + 1]?.id ?? state.document.scenes[0]?.id;
      if (["object.show","object.hide","object.setText"].includes(action.type)) action.target = scene().objects[0]?.id;
      if (action.type === "object.setText") action.value = "";
      item.actions = [action];
      renderInteractions();
    });
  });

  interactionList.querySelectorAll(".interaction-target").forEach(select => {
    select.addEventListener("change", () => {
      const item = scene().interactions.find(value => value.id === select.dataset.id);
      if (item) item.actions[0].target = select.value;
    });
    const action = scene().interactions.find(value => value.id === select.dataset.id)?.actions[0];
    if (action?.target) select.value = action.target;
  });

  interactionList.querySelectorAll(".interaction-value").forEach(input => {
    input.addEventListener("input", () => {
      const item = scene().interactions.find(value => value.id === input.dataset.id);
      if (item) item.actions[0].value = input.value;
    });
  });

  interactionList.querySelectorAll(".remove-interaction").forEach(button => {
    button.addEventListener("click", () => removeInteraction(button.dataset.id));
  });
}

function render(keepSelection = true) {
  stage.querySelectorAll(".syn-object").forEach(el => el.remove());
  const activeScene = scene();
  emptyState.hidden = activeScene.objects.length > 0;
  sceneLabel.textContent = activeScene.name;
  document.querySelector("#prevScene").disabled = state.sceneIndex === 0;
  document.querySelector("#nextScene").disabled = state.sceneIndex === state.document.scenes.length - 1;
  for (const object of activeScene.objects) {
    const el = document.createElement("button");
    el.className = "syn-object syn-" + object.kind;
    el.dataset.id = object.id;
    el.dataset.kind = object.kind;
    el.textContent = object.props?.text || object.label;
    if (object.styles) {
      const supported = {
        fontFamily:"fontFamily",fontSize:"fontSize",fontWeight:"fontWeight",color:"color",
        background:"background",borderRadius:"borderRadius",borderColor:"borderColor",
        borderWidth:"borderWidth",boxShadow:"boxShadow",letterSpacing:"letterSpacing",
        lineHeight:"lineHeight",opacity:"opacity",textAlign:"textAlign",padding:"padding",
        textTransform:"textTransform"
      };
      for (const [key,value] of Object.entries(object.styles)) {
        const prop = supported[key];
        if (!prop || value == null) continue;
        el.style[prop] = typeof value === "number" && ["fontSize","borderRadius","borderWidth"].includes(key) ? value + "px" : value;
      }
    }
    el.style.left = object.x + "px";
    el.style.top = object.y + "px";
    el.style.width = object.width + "px";
    el.style.height = object.height + "px";
    el.addEventListener("click", event => { event.stopPropagation(); selectObject(object.id); });
    el.addEventListener("dblclick", event => {
      if (object.kind !== "text") return;
      event.stopPropagation();
      const input = document.createElement("textarea");
      input.className = "inline-edit"; input.value = object.props?.text ?? object.label;
      input.style.left = object.x + "px"; input.style.top = object.y + "px"; input.style.width = object.width + "px"; input.style.height = object.height + "px";
      input.style.fontFamily = object.styles?.fontFamily || "Inter"; input.style.fontSize = (object.styles?.fontSize || 24) + "px";
      stage.appendChild(input); input.focus(); input.select();
      const commit = () => { object.props = { ...object.props, text: input.value }; input.remove(); render(); };
      input.addEventListener("blur", commit, { once: true });
      input.addEventListener("keydown", event => { if (event.key === "Escape") { input.remove(); render(); } if (event.key === "Enter" && !event.shiftKey) { event.preventDefault(); commit(); } });
    });
    el.addEventListener("pointerdown", event => beginDrag(event, object));
    stage.appendChild(el);
  }
  renderInteractions();
  if (keepSelection && state.selectedObjectId && selectObject) selectObject(state.selectedObjectId);
}
function beginDrag(event, object) {
  if (event.button !== 0) return;
  event.preventDefault();
  const rect = stage.getBoundingClientRect();
  state.drag = { object, startX: event.clientX, startY: event.clientY, originX: object.x, originY: object.y, rect };
  selectObject(object.id);
  const move = e => {
    if (!state.drag) return;
    const d = state.drag;
    d.object.x = Math.max(0, Math.round(d.originX + e.clientX - d.startX));
    d.object.y = Math.max(0, Math.round(d.originY + e.clientY - d.startY));
    render(false);
  };
  const end = () => {
    state.drag = null;
    window.removeEventListener("pointermove", move);
    window.removeEventListener("pointerup", end);
    selectObject(object.id);
  };
  window.addEventListener("pointermove", move);
  window.addEventListener("pointerup", end, { once: true });
}

function preview() {
  const runtime = createRuntimeState(structuredClone(state.document));
  const win = window.open("", "_blank");
  if (!win) return alert("Allow pop-ups to preview SYN.");
  const html = `<!doctype html><html><head><title>SYN Preview</title><style>${previewCss()}</style></head><body>
    <header><strong id="title"></strong><span id="scene"></span></header><main id="stage"></main>
  </body></html>`;
  win.document.open();
  win.document.write(html);
  win.document.close();
  const mount = () => {
    if (!win.document.querySelector("#stage")) return;
    const root = win.document.querySelector("#stage");
    win.document.querySelector("#title").textContent = runtime.document.meta.title;
    win.document.querySelector("#scene").textContent = currentScene(runtime)?.name ?? "";
    renderScene(root, runtime, { onEvent: event => { dispatchEvent(runtime, event); mount(); } });
  };
  setTimeout(mount, 0);
}

function previewCss() {
  return `body{margin:0;background:#090a0d;color:#fff;font-family:system-ui}header{height:56px;padding:0 18px;display:flex;align-items:center;justify-content:space-between;border-bottom:1px solid #242833}main{position:relative;margin:20px;min-height:600px;border:1px solid #2b2f39;border-radius:12px;background:#0c0e13;background-image:linear-gradient(#151821 1px,transparent 1px),linear-gradient(90deg,#151821 1px,transparent 1px);background-size:24px 24px}.syn-runtime-object{position:absolute;border:1px solid #3a4050;border-radius:8px;background:#171a22;color:#fff;padding:10px 14px;white-space:pre-wrap;overflow:hidden}.syn-runtime-object[data-kind=button]{background:#ff4fd8;color:#160b14;border-color:transparent;font-weight:800;cursor:pointer;text-align:center}`;
}

function exportSyn() {
  const source = serializeSynDocument(state.document);
  const blob = new Blob([source], { type: "application/vnd.syn+json" });
  const url = URL.createObjectURL(blob);
  const anchor = Object.assign(window.document.createElement("a"), {
    href: url,
    download: (state.document.meta.title || "untitled").replace(/[^a-z0-9_-]+/gi, "-").toLowerCase() + ".syn"
  });
  anchor.click();
  setTimeout(() => URL.revokeObjectURL(url), 1000);
}

function escapeHtml(value) {
  return String(value).replace(/[&<>"']/g, char => ({ "&":"&amp;", "<":"&lt;", ">":"&gt;", '"':"&quot;", "'":"&#39;" }[char]));
}

document.querySelector("#mediaInput").addEventListener("change", async event => {
  const file = event.target.files?.[0];
  if (!file) return;
  if (!file.type.startsWith("image/")) {
    alert("SYN currently embeds images as media assets.");
    event.target.value = "";
    return;
  }
  const reader = new FileReader();
  reader.onload = () => {
    const count = scene().objects.length;
    const object = addObject(scene(), {
      kind: "media",
      label: file.name.replace(/\.[^.]+$/, "") || "Image",
      x: 60 + (count % 4) * 40,
      y: 60 + (count % 4) * 40,
      width: 220,
      height: 160,
      props: { src: String(reader.result) }
    });
    state.selectedObjectId = object.id;
    render();
  };
  reader.readAsDataURL(file);
  event.target.value = "";
});

document.querySelectorAll(".tool").forEach(button => {
  button.addEventListener("click", () => {
    document.querySelectorAll(".tool").forEach(item => item.classList.remove("active"));
    button.classList.add("active");
    const tool = button.dataset.tool;
    if (["text","media","shape","button"].includes(tool)) addCanvasObject(tool);
    if (tool === "scene") addNewScene();
    if (tool === "ai") alert("SYN AI will edit this document model directly. Runtime and authoring foundations come first.");
  });
});
stage.addEventListener("click", () => selectObject(null));
deleteButton.addEventListener("click", deleteSelectedObject);
document.querySelector("#addInteraction").addEventListener("click", addNewInteraction);
document.querySelector("#export").addEventListener("click", exportSyn);
document.querySelector("#newScene").addEventListener("click", addNewScene);
document.querySelector("#newDocument").addEventListener("click", () => {
  if (!confirm("Start a new SYN document? Unsaved work will be lost.")) return;
  state.document = createSynDocument({ title: "Untitled SYN" });
  addScene(state.document, { name: "Scene 1" });
  state.sceneIndex = 0;
  state.selectedObjectId = null;
  render();
});
document.querySelector("#openSyn").addEventListener("change", async event => {
  const file = event.target.files?.[0];
  if (!file) return;
  try {
    const source = await file.text();
    const runtime = createRuntimeState(JSON.parse(source));
    state.document = runtime.document;
    state.sceneIndex = 0;
    state.selectedObjectId = null;
    render();
  } catch (error) {
    alert(error instanceof Error ? error.message : "Unable to open SYN document.");
  }
  event.target.value = "";
});

document.querySelector("#preview").addEventListener("click", preview);
buildApplicationMenus();
document.addEventListener("keydown",e=>{if((e.metaKey||e.ctrlKey)&&e.key.toLowerCase()==="k"){e.preventDefault();openCodePalette()}});
document.querySelector("#prevScene").addEventListener("click", () => changeScene(-1));
document.querySelector("#nextScene").addEventListener("click", () => changeScene(1));
render();
