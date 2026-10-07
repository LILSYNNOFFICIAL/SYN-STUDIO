import { createSynDocument, addScene, addObject, addInteraction, serializeSynDocument } from "../src/document.js";
import { createRuntimeState, currentScene, dispatchEvent, renderScene, responsiveScale } from "../src/runtime.js";
import { createHistory } from "../src/history.js";
import { moveObject, resizeObject, duplicateObject, reorderObject } from "../src/editor.js";

const state = {
  document: createSynDocument({ title: "SYN Studio Showcase" }),
  sceneIndex: 0,
  selectedObjectId: null,
  drag: null,
  history: null,
  zoom: 1,
  selectedObjectIds: [],
  toolMode: "select",
  workspaceMode: "design",
  pan: { x: 0, y: 0 },
  commandSettings: {},
  clipboard: null,
  snap: true,
  guides: false,
  rulers: false
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

card(source,"src-editor","",72,205,760,335,{background:"#090d13",borderColor:"#2d3949",borderWidth:1,borderRadius:14,boxShadow:"0 24px 60px #0009"});
text(source,"src-code-1",'{  "syn": "0.1",',98,232,620,22,{fontFamily:"ui-monospace, SFMono-Regular, Consolas, monospace",fontSize:12,color:"#c8d2e3"});
text(source,"src-code-2",'  "type": "document",',98,258,620,22,{fontFamily:"ui-monospace, SFMono-Regular, Consolas, monospace",fontSize:12,color:"#c8d2e3"});
text(source,"src-code-3",'  "scenes": [',98,284,620,22,{fontFamily:"ui-monospace, SFMono-Regular, Consolas, monospace",fontSize:12,color:"#8faeff"});
text(source,"src-code-4",'    { "id": "scene-3",',98,310,620,22,{fontFamily:"ui-monospace, SFMono-Regular, Consolas, monospace",fontSize:12,color:"#c8d2e3"});
text(source,"src-code-5",'      "objects": [ ... ],',98,336,620,22,{fontFamily:"ui-monospace, SFMono-Regular, Consolas, monospace",fontSize:12,color:"#9ce5c0"});
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
const MENU_GROUPS = {
  File: ["File"],
  Edit: ["Edit"],
  View: ["View"],
  Insert: ["Insert"],
  Design: ["Format","Typography","Effects","Accessibility","Localization"],
  Arrange: ["Arrange","Object","Layout","Responsive"],
  Scene: ["Project","Scene","Timeline","Animation"],
  Media: ["Media","Audio","Video","Assets"],
  Interact: ["Interaction","Navigation","Logic","State","Signals"],
  Data: ["Variables","Data","Database","API","Forms"],
  Code: ["Web","Code","Debug"],
  AI: ["AI"],
  Publish: ["Version","Collaboration","Build","Package","Publish"],
  Tools: ["Components","Symbols","Performance","Security","Tools","Window"],
  Help: ["Help"]
};

function flattenMenuItems(){
  return Object.entries(MENU_DATA).flatMap(([menu,items]) => items.map(name => ({ menu, name })));
}

function buildApplicationMenus(){
  const nav=document.createElement("nav");
  nav.className="menu-bar";
  nav.setAttribute("aria-label","SYN Studio application menu");
  nav.innerHTML=Object.entries(MENU_GROUPS).map(([name,menus])=>{
    const sections=menus.map(section=>{
      const items=MENU_DATA[section]||[];
      return '<section class="menu-section"><div class="menu-section-title">'+escMenu(section)+'</div>'+
        items.map(item=>'<button class="menu-item" data-command="'+escMenu(item)+'" data-menu="'+escMenu(section)+'">'+escMenu(item)+'</button>').join("")+
      '</section>';
    }).join("");
    return '<div class="menu"><button class="menu-trigger" aria-haspopup="true" aria-expanded="false">'+escMenu(name)+'</button><div class="menu-panel">'+sections+'</div></div>';
  }).join("");
  document.querySelector(".brand").after(nav);

  // On phones, keep high-frequency menus visible and move the rest into one overflow menu.
  const mobileMore=document.createElement("div");
  mobileMore.className="menu mobile-more-menu";
  mobileMore.innerHTML='<button class="menu-trigger mobile-more-trigger" aria-haspopup="true" aria-expanded="false">More <span aria-hidden="true">⌄</span></button><div class="menu-panel mobile-more-panel"></div>';
  const mobilePanel=mobileMore.querySelector(".menu-panel");
  mobilePanel.innerHTML=Object.entries(MENU_DATA).map(([section,items])=>
    '<section class="menu-section"><div class="menu-section-title">'+escMenu(section)+'</div>'+
    items.map(item=>'<button class="menu-item" data-command="'+escMenu(item)+'" data-menu="'+escMenu(section)+'">'+escMenu(item)+'</button>').join("")+
    '</section>'
  ).join("");
  nav.appendChild(mobileMore);

  const closeMenus=except=>{
    nav.querySelectorAll(".menu.open").forEach(m=>{
      if(m!==except){
        m.classList.remove("open");
        m.querySelector(".menu-trigger")?.setAttribute("aria-expanded","false");
      }
    });
  };

  const positionPanel=(menu,button)=>{
    const panel=menu.querySelector(".menu-panel");
    if(!panel)return;
    const r=button.getBoundingClientRect();
    const width=Math.min(520,Math.max(300,window.innerWidth-24));
    const left=Math.min(Math.max(12,r.left),Math.max(12,window.innerWidth-width-12));
    const top=Math.min(window.innerHeight-24,r.bottom+5);
    panel.style.width=width+"px";
    panel.style.left=left+"px";
    panel.style.top=top+"px";
  };

  nav.querySelectorAll(".menu-trigger").forEach(button=>button.addEventListener("click",event=>{
    event.stopPropagation();
    const menu=button.parentElement;
    const opening=!menu.classList.contains("open");
    closeMenus(menu);
    menu.classList.toggle("open",opening);
    button.setAttribute("aria-expanded",String(opening));
    if(opening)positionPanel(menu,button);
  }));

  nav.querySelectorAll(".menu-item").forEach(button=>button.addEventListener("click",()=>{
    runMenuCommand(button.dataset.command,button.dataset.menu);
  }));

  mobileMore.querySelector(".menu-trigger").addEventListener("click",event=>{
    event.stopPropagation();
    const opening=!mobileMore.classList.contains("open");
    closeMenus(mobileMore);
    mobileMore.classList.toggle("open",opening);
    mobileMore.querySelector(".menu-trigger").setAttribute("aria-expanded",String(opening));
    if(opening)positionPanel(mobileMore,mobileMore.querySelector(".menu-trigger"));
  });

  window.addEventListener("resize",()=>{
    nav.querySelectorAll(".menu.open").forEach(menu=>positionPanel(menu,menu.querySelector(".menu-trigger")));
  });
  document.addEventListener("click",event=>{
    if(!event.target.closest(".menu"))closeMenus(null);
  });
  document.addEventListener("keydown",event=>{
    if(event.key==="Escape")closeMenus(null);
  });
}

function runMenuCommand(command,sourceMenu=""){
  document.querySelectorAll(".menu.open").forEach(x=>{
    x.classList.remove("open");
    x.querySelector(".menu-trigger")?.setAttribute("aria-expanded","false");
  });

  const direct={
    "New SYN":()=>document.querySelector("#newDocument")?.click(),
    "Open":()=>document.querySelector("#openSyn")?.click(),
    "Save":()=>exportSyn(),
    "Save As":()=>exportSyn(),
    "Export":()=>exportSyn(),
    "Export SYN":()=>exportSyn(),
    "Publish SYN":()=>exportSyn(),
    "Delete":()=>deleteSelectedObject(),
    "Undo":()=>undo(),
    "Redo":()=>redo(),
    "Duplicate":()=>duplicateSelected(),
    "Grid":()=>stage.classList.toggle("no-grid"),
    "Fit Canvas":()=>fitCanvas(),
    "Fullscreen":()=>document.documentElement.requestFullscreen?.(),
    "Layers":()=>openLayersPanel(),
    "Show Layers":()=>openLayersPanel(),
    "Show Assets":()=>openAssetBrowser(),
    "Assets":()=>openAssetBrowser(),
    "Asset Library":()=>openAssetBrowser(),
    "Media Library":()=>openAssetBrowser(),
    "Image":()=>document.querySelector("#mediaInput")?.click(),
    "Import":()=>document.querySelector("#mediaInput")?.click(),
    "Import Media":()=>document.querySelector("#mediaInput")?.click(),
    "Preview":()=>preview(),
    "Code Editor":()=>openCodeWorkspace(),
    "Show Code":()=>openCodeWorkspace(),
    "Source Viewer":()=>openCodeWorkspace(),
    "JavaScript":()=>openCodeWorkspace(),
    "Command Palette":()=>openCodePalette(),
    "Align Left":()=>alignSelected("left"),
    "Align Center":()=>alignSelected("centerX"),
    "Align Right":()=>alignSelected("right"),
    "Align Top":()=>alignSelected("top"),
    "Align Middle":()=>alignSelected("centerY"),
    "Align Bottom":()=>alignSelected("bottom"),
    "Distribute Horizontally":()=>distributeSelected("x"),
    "Distribute Vertically":()=>distributeSelected("y"),
    "Bring to Front":()=>moveSelectedLayer("front"),
    "Send to Back":()=>moveSelectedLayer("back"),
    "Lock":()=>toggleSelectedLock(),
    "Unlock":()=>toggleSelectedLock(),
    "Hide":()=>toggleSelectedVisibility(),
    "Show":()=>toggleSelectedVisibility(),
    "New Scene":()=>addNewScene(),
    "New from Template":()=>addNewScene("Template Scene"),
    "Duplicate Scene":()=>duplicateCurrentScene(),
    "Delete Scene":()=>deleteCurrentScene(),
    "Rename Scene":()=>renameCurrentScene(),
    "Rename":()=>renameSelectedObject(),
    "Object Properties":()=>selectObject(state.selectedObjectId),
    "Inspector":()=>selectObject(state.selectedObjectId),
    "Object Inspector":()=>selectObject(state.selectedObjectId),
    "Document Inspector":()=>renderDocumentInspector(),
    "Fit Selection":()=>fitSelection(),
    "Zoom In":()=>setZoom(state.zoom+0.1),
    "Zoom Out":()=>setZoom(state.zoom-0.1),
    "Actual Size":()=>setZoom(1),
    "Show Timeline":()=>openCapabilityPanel("Timeline"),
    "Timeline":()=>openCapabilityPanel("Timeline"),
    "Show Interactions":()=>openCapabilityPanel("Interactions"),
    "Text":()=>addCanvasObject("text","Text"),
    "Rich Text":()=>addCanvasObject("text","Rich Text"),
    "Heading":()=>addCanvasObject("text","Heading"),
    "Paragraph":()=>addCanvasObject("text","Paragraph"),
    "Markdown":()=>addCanvasObject("text","Markdown"),
    "Dynamic Text":()=>addCanvasObject("text","Dynamic Text"),
    "Code Block":()=>addCanvasObject("text","Code Block"),
    "Link":()=>addCanvasObject("button","Link"),
    "Button":()=>addCanvasObject("button","Button"),
    "Shape":()=>addCanvasObject("shape","Shape"),
    "Line":()=>addCanvasObject("shape","Line"),
    "Arrow":()=>addCanvasObject("shape","Arrow"),
    "Component":()=>addCanvasObject("shape","Component"),
    "Image":()=>document.querySelector("#mediaInput")?.click(),
    "SVG":()=>document.querySelector("#mediaInput")?.click(),
    "GIF":()=>document.querySelector("#mediaInput")?.click(),
    "Audio":()=>document.querySelector("#mediaInput")?.click(),
    "Video":()=>document.querySelector("#mediaInput")?.click(),
    "Gallery":()=>openCapabilityPanel("Gallery"),
    "Slideshow":()=>openCapabilityPanel("Slideshow"),
    "Embed":()=>openCapabilityPanel("Embed"),
    "Web Content":()=>openCapabilityPanel("Web Content"),
    "Form":()=>addCanvasObject("shape","Form"),
    "Input":()=>addCanvasObject("shape","Input"),
    "Checkbox":()=>addCanvasObject("shape","Checkbox"),
    "Toggle":()=>addCanvasObject("shape","Toggle"),
    "Dropdown":()=>addCanvasObject("shape","Dropdown"),
    "Menu":()=>addCanvasObject("shape","Menu"),
    "Icon":()=>addCanvasObject("shape","Icon"),
    "QR Code":()=>addCanvasObject("shape","QR Code"),
    "Map":()=>addCanvasObject("shape","Map"),
    "Chart":()=>addCanvasObject("shape","Chart"),
    "Table":()=>addCanvasObject("shape","Table"),
    "Timer":()=>addCanvasObject("shape","Timer"),
    "Scene":()=>addNewScene(),
    "Hotspot":()=>addCanvasObject("shape","Hotspot"),
    "Popup":()=>openCapabilityPanel("Popup"),
    "Modal":()=>openCapabilityPanel("Modal"),
    "Tooltip":()=>openCapabilityPanel("Tooltip"),
    "Animation":()=>openCapabilityPanel("Animation"),
    "Interactions":()=>openCapabilityPanel("Interactions"),
    "Select All":()=>{state.selectedObjectIds=scene().objects.filter(o=>!o.hidden).map(o=>o.id);state.selectedObjectId=state.selectedObjectIds.at(-1)||null;render();},
    "Select None":()=>{state.selectedObjectIds=[];state.selectedObjectId=null;render();},
    "Select Similar":()=>{const object=selected();if(!object)return;state.selectedObjectIds=scene().objects.filter(o=>o.kind===object.kind).map(o=>o.id);state.selectedObjectId=state.selectedObjectIds.at(-1)||null;render();},
    "Copy":()=>{const object=selected();if(object)state.clipboard=structuredClone(object);showToast(object?"Copied selected object.":"Select an object first.");},
    "Cut":()=>{const object=selected();if(!object){showToast("Select an object first.");return;}state.clipboard=structuredClone(object);deleteSelectedObject();},
    "Paste":()=>pasteClipboard(false),
    "Paste in Place":()=>pasteClipboard(true),
    "Rulers":()=>{state.rulers=!state.rulers;stage.classList.toggle("show-rulers",state.rulers);showToast("Rulers "+(state.rulers?"on":"off"));},
    "Guides":()=>{state.guides=!state.guides;stage.classList.toggle("show-guides",state.guides);showToast("Guides "+(state.guides?"on":"off"));},
    "Snap":()=>{state.snap=!state.snap;showToast("Snap "+(state.snap?"on":"off"));},
    "Reset Workspace":()=>{closeCodeWorkspace();state.toolMode="select";state.pan={x:0,y:0};state.selectedObjectIds=[];state.selectedObjectId=null;fitCanvas();},
    "Presentation Mode":()=>preview(),
    "Open Recent":()=>openCapabilityPanel("Open Recent"),
    "Save a Copy":()=>exportSyn(),
    "Auto Save":()=>{state.commandSettings["Auto Save"]=state.commandSettings["Auto Save"]==="on"?"off":"on";showToast("Auto Save "+state.commandSettings["Auto Save"]);},
    "Fit":()=>fitCanvas()
  };
  if(direct[command]){ direct[command](); return; }

  const lower=command.toLowerCase();
  if(/^(text|rich text|heading|paragraph|markdown|dynamic text|code block)$/.test(lower)){
    addCanvasObject("text",command);
    return;
  }
  if(/^(button)$/.test(lower)){
    addCanvasObject("button");
    return;
  }
  if(/^(shape|line|arrow|rectangle|circle|path)$/.test(lower)){
    addCanvasObject("shape",command);
    return;
  }
  if(/^(component|symbol|create component|create symbol|graphic symbol|movie symbol|interactive symbol|button symbol)$/.test(lower)){
    addCanvasObject("shape",command);
    const object=selected();
    if(object)object.props={...object.props,componentType:command};
    render();
    return;
  }
  if(/^(audio|video|gif|svg|gallery|slideshow|embed|web content|external assets|documents)$/.test(lower) || lower.includes("import audio") || lower.includes("import video")){
    document.querySelector("#mediaInput")?.click();
    return;
  }
  if(lower.includes("font") || lower.includes("typography") || lower.includes("color") || lower.includes("gradient") || lower.includes("shadow") || lower.includes("opacity") || lower.includes("alignment") || lower.includes("line height") || lower.includes("letter spacing")){
    if(selected()) addStyleInspector(); else openCapabilityPanel(command,sourceMenu);
    return;
  }
  if(lower.includes("code") || lower.includes("source") || lower.includes("javascript") || lower.includes("formatter") || lower.includes("linter") || lower.includes("type checker")){
    openCodeWorkspace();
    return;
  }
  if(lower.includes("debug") || lower.includes("console") || lower.includes("runtime") || lower.includes("error") || lower.includes("watch")){
    openCapabilityPanel(command);
    return;
  }
  if(lower.includes("api") || lower.includes("database") || lower.includes("data") || lower.includes("form") || lower.includes("variable") || lower.includes("state") || lower.includes("signal")){
    openCapabilityPanel(command);
    return;
  }
  if(lower.includes("preview") || lower.includes("presentation")){
    preview();
    return;
  }
  if(lower.includes("delete") || lower.includes("remove")){
    deleteSelectedObject();
    return;
  }
  if(lower.includes("new ") || lower.startsWith("create ") || lower.startsWith("add ")){
    addCanvasObject("shape",command);
    return;
  }
  openCapabilityPanel(command,sourceMenu);
}

function openCapabilityPanel(command,sourceMenu=""){
  let panel=document.querySelector("#syn-capability-panel");
  if(!panel){
    panel=document.createElement("section");
    panel.id="syn-capability-panel";
    panel.className="modal";
    panel.innerHTML='<div class="capability-card"><div class="capability-head"><div><div class="eyebrow">SYN TOOL</div><h2 id="capability-title"></h2><p id="capability-description"></p></div><button id="close-capability">Close</button></div><div class="capability-body"><label class="field">Configuration<textarea id="capability-value" rows="7" placeholder="Configure this command for the current authoring session..."></textarea></label><div class="capability-actions"><button id="capability-reset">Reset</button><button id="capability-apply" class="primary">Apply</button></div><div id="capability-status" class="capability-status"></div></div></div>';
    document.body.appendChild(panel);
    panel.querySelector("#close-capability").addEventListener("click",()=>panel.hidden=true);
    panel.addEventListener("click",event=>{if(event.target===panel)panel.hidden=true});
    panel.querySelector("#capability-reset").addEventListener("click",()=>{panel.querySelector("#capability-value").value=""});
    panel.querySelector("#capability-apply").addEventListener("click",()=>{
      const key=panel.dataset.command||"tool";
      state.commandSettings[key]=panel.querySelector("#capability-value").value;
      const object=selected();
      if(object){
        object.props={...object.props,toolSettings:{...(object.props?.toolSettings||{}),[key]:panel.querySelector("#capability-value").value}};
        recordHistory();
        render();
      }
      panel.querySelector("#capability-status").textContent="Configuration applied to this authoring session"+(object?" and selected object.":".");
    });
  }
  panel.dataset.command=command;
  panel.querySelector("#capability-title").textContent=command;
  panel.querySelector("#capability-description").textContent=(sourceMenu?sourceMenu+" • ":"")+"This command is connected to the SYN authoring surface. Configure it here without leaving the document.";
  panel.querySelector("#capability-value").value=state.commandSettings[command]||"";
  panel.querySelector("#capability-status").textContent="";
  panel.hidden=false;
}

function showToast(message){let t=document.querySelector(".toast");if(!t){t=document.createElement("div");t.className="toast";document.body.appendChild(t)}t.textContent=message;t.classList.add("show");clearTimeout(showToast.timer);showToast.timer=setTimeout(()=>t.classList.remove("show"),2200)}
function openCodePalette(){let p=document.querySelector("#syn-command-palette");if(!p){p=document.createElement("div");p.id="syn-command-palette";p.className="modal";p.innerHTML='<div class="palette"><input id="command-search" placeholder="Search commands, tools, scenes..."><div id="command-results"></div></div>';document.body.appendChild(p);p.onclick=e=>{if(e.target===p)p.hidden=true};p.querySelector("input").addEventListener("input",renderCommandResults)}p.hidden=false;p.querySelector("input").focus();renderCommandResults()}
function renderCommandResults(){
  const p=document.querySelector("#syn-command-palette");
  const q=p.querySelector("input").value.toLowerCase();
  const rows=flattenMenuItems().filter(x=>!q||(x.name+" "+x.menu).toLowerCase().includes(q)).slice(0,80);
  p.querySelector("#command-results").innerHTML=rows.map(x=>'<button class="command-row" data-command="'+escMenu(x.name)+'"><span>'+escMenu(x.name)+'</span><small>'+escMenu(x.menu)+'</small></button>').join("");
  p.querySelectorAll(".command-row").forEach(b=>b.addEventListener("click",()=>{p.hidden=true;runMenuCommand(b.dataset.command)}));
}
function setWorkspaceMode(mode){
  state.workspaceMode=mode;
  document.body.classList.toggle("code-mode",mode==="code");
  document.querySelectorAll("[data-workspace-mode]").forEach(button=>button.classList.toggle("active",button.dataset.workspaceMode===mode));
  if(mode!=="code"){
    const panel=document.querySelector("#syn-code-workspace");
    if(panel)panel.hidden=true;
  }
  if(mode==="code")openCodeWorkspace();
}

function closeCodeWorkspace(){
  const panel=document.querySelector("#syn-code-workspace");
  if(panel)panel.hidden=true;
  document.body.classList.remove("code-mode");
  state.workspaceMode="design";
  document.querySelectorAll("[data-workspace-mode]").forEach(button=>button.classList.toggle("active",button.dataset.workspaceMode==="design"));
}

function openCodeWorkspace(){
  let panel=document.querySelector("#syn-code-workspace");
  if(!panel){
    panel=document.createElement("section");
    panel.id="syn-code-workspace";
    panel.className="code-workspace";
    panel.innerHTML='<div class="code-head"><div><div class="eyebrow">SOURCE</div><strong>SYN document code</strong><span class="code-subtitle">Structured source for the current project</span></div><div class="code-actions"><input id="syn-source-search" placeholder="Find in source"><button id="validate-syn-source">Validate</button><button id="format-syn-source">Format</button><button id="close-syn-source">Design</button><button id="apply-syn-source" class="primary">Apply</button></div></div><div class="code-editor-shell"><pre id="syn-source-lines" aria-hidden="true">1</pre><textarea id="syn-source" spellcheck="false" aria-label="SYN source editor"></textarea></div><div class="code-foot"><span id="syn-source-status">Editing the SYN source directly. Canvas manipulation is disabled in Code mode.</span><span>Ctrl/Cmd+K opens commands</span></div>';
    document.querySelector(".stage-area").appendChild(panel);
    panel.querySelector("#format-syn-source").addEventListener("click",()=>{
      try {
        panel.querySelector("#syn-source").value=JSON.stringify(JSON.parse(panel.querySelector("#syn-source").value),null,2)+"\n";
        panel.querySelector("#syn-source-status").textContent="Formatted.";
        syncCodeLines(panel);
      } catch(error) {
        panel.querySelector("#syn-source-status").textContent=error.message;
      }
    });
    panel.querySelector("#validate-syn-source").addEventListener("click",()=>{
      try {
        createRuntimeState(JSON.parse(panel.querySelector("#syn-source").value));
        panel.querySelector("#syn-source-status").textContent="Valid SYN document.";
      } catch(error) {
        panel.querySelector("#syn-source-status").textContent="Validation error: "+error.message;
      }
    });
    panel.querySelector("#close-syn-source").addEventListener("click",closeCodeWorkspace);
    const source=panel.querySelector("#syn-source");
    source.addEventListener("input",()=>syncCodeLines(panel));
    source.addEventListener("scroll",()=>{panel.querySelector("#syn-source-lines").scrollTop=source.scrollTop;});
    panel.querySelector("#syn-source-search").addEventListener("input",event=>{
      const q=event.target.value;
      if(!q){source.focus();return;}
      const index=source.value.toLowerCase().indexOf(q.toLowerCase());
      if(index>=0){source.focus();source.setSelectionRange(index,index+q.length);}
    });
    panel.querySelector("#apply-syn-source").addEventListener("click",()=>{
      try {
        const parsed=JSON.parse(source.value);
        createRuntimeState(parsed);
        const previousDocument=snapshotDocument();
        state.document=parsed;
        state.sceneIndex=0;
        state.selectedObjectId=null;
        state.selectedObjectIds=[];
        state.pan={x:0,y:0};
        state.history=createHistory(previousDocument,{limit:100});
        state.history.push(previousDocument);
        render();
        panel.querySelector("#syn-source-status").textContent="Applied and validated.";
      } catch(error) {
        panel.querySelector("#syn-source-status").textContent="Cannot apply: "+error.message;
      }
    });
  }
  panel.hidden=false;
  document.body.classList.add("code-mode");
  state.workspaceMode="code";
  document.querySelectorAll("[data-workspace-mode]").forEach(button=>button.classList.toggle("active",button.dataset.workspaceMode==="code"));
  panel.querySelector("#syn-source").value=serializeSynDocument(state.document);
  syncCodeLines(panel);
  panel.querySelector("#syn-source").focus();
}

function syncCodeLines(panel){
  const source=panel.querySelector("#syn-source");
  panel.querySelector("#syn-source-lines").textContent=Array.from({length:Math.max(1,source.value.split("\n").length)},(_,i)=>i+1).join("\n");
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

function addCanvasObject(kind, variant="") {
  if (kind === "media") {
    document.querySelector("#mediaInput").click();
    return;
  }
  recordHistory();
  const labels = { text: "Text", media: "Media", shape: "Shape", button: "Button" };
  const count = scene().objects.length;
  const textVariants={
    "Rich Text":{text:"Rich text",fontSize:20,fontWeight:500,lineHeight:1.5},
    "Heading":{text:"Heading",fontSize:36,fontWeight:800},
    "Paragraph":{text:"Paragraph text",fontSize:16,fontWeight:400,lineHeight:1.55},
    "Markdown":{text:"**Markdown**",fontSize:16,fontWeight:400},
    "Code Block":{text:"const syn = true;",fontSize:13,fontFamily:"ui-monospace, SFMono-Regular, Consolas, monospace",background:"#0a1018",borderRadius:8},
    "Dynamic Text":{text:"Dynamic text",fontSize:18,fontWeight:600}
  };
  const variantStyle=textVariants[variant]||{fontSize:16};
  const object = addObject(scene(), {
    kind,
    label: variant || labels[kind] || "Object",
    x: 60 + (count % 5) * 34,
    y: 60 + (count % 5) * 34,
    width: kind === "text" ? 260 : 160,
    height: kind === "text" ? 72 : 56,
    props: kind === "text" ? { text: textVariants[variant]?.text || "Double-click to edit", ...(variant?{textType:variant}: {}) } : {}
  });
  object.styles={...object.styles,...variantStyle};
  state.selectedObjectId = object.id;
  state.selectedObjectIds = [object.id];
  render();
}

function renderDocumentInspector() {
  inspector.innerHTML = `
    <div class="inspector-heading"><div><div class="eyebrow">DOCUMENT</div><strong>${escapeHtml(state.document.meta.title || "Untitled SYN")}</strong></div><span class="inspector-badge">SYN</span></div>
    <details class="inspector-section" open><summary>Project</summary><div class="inspector-section-body">
      <label class="field">Title<input id="documentTitle" value="${escapeHtml(state.document.meta.title)}"></label>
      <label class="field">Scene name<input id="sceneNameInput" value="${escapeHtml(scene().name)}"></label>
    </div></details>
    <details class="inspector-section" open><summary>Scene</summary><div class="inspector-section-body">
      <div class="stat-grid"><div><span>Scenes</span><strong>${state.document.scenes.length}</strong></div><div><span>Objects</span><strong>${scene().objects.length}</strong></div></div>
    </div></details>
    <div class="inspector-tip">Tip: <kbd>V</kbd> select · <kbd>M</kbd> move · <kbd>H</kbd> pan · <kbd>Enter</kbd> edit text</div>
  `;
  inspector.querySelector("#documentTitle").addEventListener("input", event => { state.document.meta.title = event.target.value || "Untitled SYN"; });
  inspector.querySelector("#sceneNameInput").addEventListener("input", event => { scene().name = event.target.value || "Scene"; sceneLabel.textContent = scene().name; });
}

function selectObject(id, additive = false) {
  if (additive && id) {
    const set = new Set(state.selectedObjectIds);
    set.has(id) ? set.delete(id) : set.add(id);
    state.selectedObjectIds = [...set];
    state.selectedObjectId = state.selectedObjectIds.at(-1) || null;
  } else {
    state.selectedObjectId = id;
    state.selectedObjectIds = id ? [id] : [];
  }
  if (id) document.body.classList.add("show-inspector");
  document.body.classList.toggle("has-selection", Boolean(id));
  document.querySelectorAll(".syn-object").forEach(el => el.classList.toggle("selected", state.selectedObjectIds.includes(el.dataset.id)));
  deleteButton.disabled = !id;
  const object = selected();
  if (!object) { renderDocumentInspector(); return; }

  inspector.innerHTML = `
    <div class="inspector-heading"><div><div class="eyebrow">OBJECT</div><strong>${escapeHtml(object.label || object.kind)}</strong></div><span class="inspector-badge">${escapeHtml(object.kind.toUpperCase())}</span></div>
    <details class="inspector-section" open><summary>Transform</summary><div class="inspector-section-body">
      <label class="field">Label<input id="objectLabel" value="${escapeHtml(object.label)}"></label>
      <div class="field-row"><label class="field">X<input id="objectX" type="number" value="${object.x}"></label><label class="field">Y<input id="objectY" type="number" value="${object.y}"></label></div>
      <div class="field-row"><label class="field">Width<input id="objectW" type="number" min="20" value="${object.width}"></label><label class="field">Height<input id="objectH" type="number" min="20" value="${object.height}"></label></div>
      <label class="field">Rotation<input id="objectRotation" type="number" min="-360" max="360" value="${object.rotation || 0}"></label>
    </div></details>
    <details class="inspector-section" open><summary>Content</summary><div class="inspector-section-body">
      <label class="field">Text<textarea id="objectText" rows="4">${escapeHtml(object.props?.text ?? "")}</textarea></label>
    </div></details>
    <details class="inspector-section" open><summary>Appearance</summary><div class="inspector-section-body">
      <div class="field-row"><label class="field">Font<select id="objectFont"><option>Inter</option><option>Georgia</option><option>Arial</option><option>Courier New</option><option>Trebuchet MS</option><option>Times New Roman</option><option>system-ui</option></select></label><label class="field">Size<input id="objectFontSize" type="number" min="8" value="${object.styles?.fontSize ?? 16}"></label></div>
      <div class="field-row"><label class="field">Weight<select id="objectFontWeight"><option value="400">Regular</option><option value="500">Medium</option><option value="600">Semibold</option><option value="700">Bold</option><option value="800">Heavy</option></select></label><label class="field">Align<select id="objectTextAlign"><option>left</option><option>center</option><option>right</option></select></label></div>
      <div class="field-row"><label class="field">Color<input id="objectColor" type="color" value="${/^#[0-9a-f]{6}$/i.test(object.styles?.color ?? "") ? object.styles.color : "#eef1f6"}"></label><label class="field">Radius<input id="objectRadius" type="number" min="0" value="${object.styles?.borderRadius ?? 0}"></label></div>
      <div class="field-row"><label class="field">Opacity<input id="objectOpacity" type="number" min="0" max="1" step="0.05" value="${object.styles?.opacity ?? 1}"></label><label class="field">Tracking<input id="objectLetterSpacing" type="number" min="-4" max="20" step="0.5" value="${object.styles?.letterSpacing ?? 0}"></label></div>
      <label class="field">Background<input id="objectBackground" value="${escapeHtml(object.styles?.background ?? "")}"></label>
    </div></details>
    <details class="inspector-section"><summary>Actions</summary><div class="inspector-section-body inspector-actions">
      <button type="button" id="duplicateSelected">Duplicate</button><button type="button" id="frontSelected">Bring front</button>
      <button type="button" id="backSelected">Send back</button><button type="button" id="lockSelected">${object.locked ? "Unlock" : "Lock"}</button>
      <button type="button" id="hideSelected">${object.hidden ? "Show" : "Hide"}</button>
    </div></details>
  `;

  const font=inspector.querySelector("#objectFont"); font.value=object.styles?.fontFamily || "Inter";
  const weight=inspector.querySelector("#objectFontWeight"); if(weight) weight.value=String(object.styles?.fontWeight || 400);
  const align=inspector.querySelector("#objectTextAlign"); if(align) align.value=object.styles?.textAlign || "left";
  inspector.querySelector("#duplicateSelected").addEventListener("click", duplicateSelected);
  inspector.querySelector("#frontSelected").addEventListener("click", () => moveSelectedLayer("front"));
  inspector.querySelector("#backSelected").addEventListener("click", () => moveSelectedLayer("back"));
  inspector.querySelector("#lockSelected").addEventListener("click", toggleSelectedLock);
  inspector.querySelector("#hideSelected").addEventListener("click", toggleSelectedVisibility);
  for (const id of ["objectLabel","objectX","objectY","objectW","objectH","objectRotation","objectText","objectFont","objectFontSize","objectFontWeight","objectTextAlign","objectColor","objectRadius","objectOpacity","objectLetterSpacing","objectBackground"]) {
    inspector.querySelector("#"+id).addEventListener("focus", () => { state._editingInspector = false; });
    inspector.querySelector("#"+id).addEventListener("input", updateSelectedObject);
  }
  inspector.addEventListener("focusout", () => { state._editingInspector = false; }, { once: true });
}

function updateSelectedObject() {
  const object = selected(); if (!object) return;
  if (!state._editingInspector) { state._editingInspector = true; recordHistory(); }
  object.label = inspector.querySelector("#objectLabel").value;
  object.x = Math.max(0, Number(inspector.querySelector("#objectX").value) || 0);
  object.y = Math.max(0, Number(inspector.querySelector("#objectY").value) || 0);
  object.width = Math.max(20, Number(inspector.querySelector("#objectW").value) || 20);
  object.height = Math.max(20, Number(inspector.querySelector("#objectH").value) || 20);
  object.rotation = Number(inspector.querySelector("#objectRotation").value) || 0;
  object.props = { ...object.props, text: inspector.querySelector("#objectText").value };
  object.styles = {
    ...object.styles,
    fontFamily: inspector.querySelector("#objectFont").value,
    fontSize: Number(inspector.querySelector("#objectFontSize").value) || 16,
    fontWeight: Number(inspector.querySelector("#objectFontWeight")?.value) || 400,
    textAlign: inspector.querySelector("#objectTextAlign")?.value || "left",
    color: inspector.querySelector("#objectColor").value,
    borderRadius: Number(inspector.querySelector("#objectRadius").value) || 0,
    opacity: Math.max(0,Math.min(1,Number(inspector.querySelector("#objectOpacity")?.value ?? 1))),
    letterSpacing: Number(inspector.querySelector("#objectLetterSpacing")?.value ?? 0),
    background: inspector.querySelector("#objectBackground").value
  };
  render(false);
}
function deleteSelectedObject() {
  const ids = new Set(state.selectedObjectIds.length ? state.selectedObjectIds : (state.selectedObjectId ? [state.selectedObjectId] : []));
  if (!ids.size) return;
  recordHistory();
  scene().objects = scene().objects.filter(object => !ids.has(object.id));
  scene().interactions = scene().interactions.filter(item => !ids.has(item.event.target) && !item.actions.some(action => ids.has(action.target)));
  state.selectedObjectId = null;
  state.selectedObjectIds = [];
  render();
}

function addNewScene() {
  recordHistory();
  const next = state.document.scenes.length + 1;
  addScene(state.document, { name: `Scene ${next}` });
  state.sceneIndex = state.document.scenes.length - 1;
  state.selectedObjectId = null;
  render();
}

function duplicateCurrentScene(){
  const current=scene();
  if(!current)return;
  recordHistory();
  const copy=structuredClone(current);
  const suffix=Date.now().toString(36);
  const idMap=new Map(current.objects.map(object=>[object.id,object.id+"-"+suffix]));
  copy.id="scene-"+suffix;
  copy.name=current.name+" Copy";
  copy.objects=copy.objects.map(object=>({...object,id:idMap.get(object.id)}));
  copy.interactions=copy.interactions.map(item=>({
    ...item,
    id:item.id+"-"+suffix,
    event:{...item.event,target:idMap.get(item.event.target) ?? item.event.target},
    actions:item.actions.map(action=>({
      ...action,
      ...(action.type === "scene.goto" && action.target === current.id ? {target:copy.id} :
        action.target && idMap.has(action.target) ? {target:idMap.get(action.target)} : {})
    }))
  }));
  state.document.scenes.splice(state.sceneIndex+1,0,copy);
  state.sceneIndex+=1;
  state.selectedObjectId=null;
  state.selectedObjectIds=[];
  render();
}
function deleteCurrentScene(){
  if(state.document.scenes.length<=1){showToast("A SYN document must keep at least one scene.");return;}
  recordHistory();
  state.document.scenes.splice(state.sceneIndex,1);
  state.sceneIndex=Math.max(0,Math.min(state.sceneIndex,state.document.scenes.length-1));
  state.selectedObjectId=null;
  state.selectedObjectIds=[];
  render();
}
function renameCurrentScene(){
  const value=window.prompt("Scene name",scene().name);
  if(value===null)return;
  recordHistory();
  scene().name=value.trim()||scene().name;
  render();
}
function renameSelectedObject(){
  const object=selected();
  if(!object){showToast("Select an object first.");return;}
  const value=window.prompt("Object label",object.label);
  if(value===null)return;
  recordHistory();
  object.label=value.trim()||object.label;
  render();
}

function changeScene(delta) {
  state.sceneIndex = Math.max(0, Math.min(state.document.scenes.length - 1, state.sceneIndex + delta));
  state.selectedObjectId = null;
  state.selectedObjectIds = [];
  document.body.classList.remove("has-selection");
  render();
}

function addNewInteraction() {
  const button = scene().objects.find(item => item.kind === "button");
  if (!button) return alert("Add a Button first.");
  recordHistory();
  const target = state.document.scenes[state.sceneIndex + 1];
  addInteraction(scene(), {
    event: { type: "click", target: button.id },
    actions: [{ type: target ? "scene.goto" : "scene.next", ...(target ? { target: target.id } : {}) }]
  });
  renderInteractions();
}

function removeInteraction(id) {
  recordHistory();
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
    const linkValue = action.type === "link.openUrl" ? action.url ?? "" : action.type === "link.openSyn" ? action.target ?? "" : "";
    return `<div class="interaction">
      <div class="interaction-rule"><strong>WHEN</strong> <span class="target-chip">${escapeHtml(source?.label ?? item.event.target)}</span> <code>clicked</code></div>
      <span class="flow">→</span>
      <div class="interaction-rule interaction-action-wrap">
        <strong>THEN</strong>
        <select class="interaction-action" data-id="${item.id}">
          <option value="scene.next" ${action.type === "scene.next" ? "selected" : ""}>Next scene</option>
          <option value="scene.goto" ${action.type === "scene.goto" ? "selected" : ""}>Go to scene</option>
          <option value="object.show" ${action.type === "object.show" ? "selected" : ""}>Show object</option>
          <option value="object.hide" ${action.type === "object.hide" ? "selected" : ""}>Hide object</option>
          <option value="object.setText" ${action.type === "object.setText" ? "selected" : ""}>Set object text</option>
          <option value="link.openUrl" ${action.type === "link.openUrl" ? "selected" : ""}>Open URL</option>
          <option value="link.openSyn" ${action.type === "link.openSyn" ? "selected" : ""}>Open SYN link</option>
        </select>
        ${action.type === "scene.goto" || ["object.show","object.hide","object.setText"].includes(action.type) ? `<select class="interaction-target" data-id="${item.id}">${targetOptions}</select>` : ""}
        ${action.type === "object.setText" ? `<input class="interaction-value" data-id="${item.id}" value="${escapeHtml(action.value ?? "")}" placeholder="Text">` : ""}
        ${action.type === "link.openUrl" ? `<input class="interaction-link" data-id="${item.id}" value="${escapeHtml(linkValue)}" placeholder="https://example.com">` : ""}
        ${action.type === "link.openSyn" ? `<input class="interaction-link" data-id="${item.id}" value="${escapeHtml(linkValue)}" placeholder="SYN URL or document target">` : ""}
      </div>
      <button class="remove-interaction" data-id="${item.id}" aria-label="Remove interaction">×</button>
    </div>`;
  }).join("");

  scene().interactions.forEach(item => {
    const actionSelect = interactionList.querySelector(`.interaction-action[data-id="${item.id}"]`);
    if (!actionSelect) return;
    actionSelect.addEventListener("change", () => {
      recordHistory();
      const action = item.actions[0] ?? { type: "scene.next" };
      action.type = actionSelect.value;
      delete action.target; delete action.value; delete action.url;
      if (action.type === "scene.goto") action.target = state.document.scenes[state.sceneIndex + 1]?.id ?? state.document.scenes[0]?.id;
      if (["object.show","object.hide","object.setText"].includes(action.type)) action.target = scene().objects[0]?.id;
      if (action.type === "object.setText") action.value = "";
      if (action.type === "link.openUrl") action.url = "https://";
      if (action.type === "link.openSyn") action.target = "";
      item.actions = [action];
      renderInteractions();
    });
  });

  interactionList.querySelectorAll(".interaction-target").forEach(select => {
    select.addEventListener("change", () => {
      recordHistory();
      const item = scene().interactions.find(value => value.id === select.dataset.id);
      if (item) item.actions[0].target = select.value;
    });
    const action = scene().interactions.find(value => value.id === select.dataset.id)?.actions[0];
    if (action?.target) select.value = action.target;
  });

  interactionList.querySelectorAll(".interaction-value").forEach(input => {
    input.addEventListener("input", () => {
      recordHistory();
      const item = scene().interactions.find(value => value.id === input.dataset.id);
      if (item) item.actions[0].value = input.value;
    });
  });

  interactionList.querySelectorAll(".interaction-link").forEach(input => {
    input.addEventListener("input", () => {
      const item = scene().interactions.find(value => value.id === input.dataset.id);
      if (!item) return;
      if (item.actions[0].type === "link.openUrl") item.actions[0].url = input.value;
      if (item.actions[0].type === "link.openSyn") item.actions[0].target = input.value;
    });
    input.addEventListener("change", () => recordHistory());
  });

  interactionList.querySelectorAll(".remove-interaction").forEach(button => {
    button.addEventListener("click", () => removeInteraction(button.dataset.id));
  });
}

function snapshotDocument() { return structuredClone(state.document); }
function recordHistory() { if (state.history) state.history.push(snapshotDocument()); }
function restoreDocument(document) {
  state.document = structuredClone(document);
  state.sceneIndex = Math.min(state.sceneIndex, Math.max(0, state.document.scenes.length - 1));
  state.selectedObjectId = null;
  state.selectedObjectIds = [];
  render(false);
}
function undo() { const previous = state.history?.undo(snapshotDocument()); if (previous) restoreDocument(previous); }
function redo() { const next = state.history?.redo(); if (next) restoreDocument(next); }
function viewport() { return state.document.viewport || { width: 1120, height: 640 }; }
function duplicateSelected() {
  const object = selected();
  if (!object) return;
  recordHistory();
  const copy = duplicateObject(scene(), object.id);
  if (copy) { state.selectedObjectId = copy.id; state.selectedObjectIds = [copy.id]; render(); }
}
function moveSelectedLayer(direction) {
  const object = selected();
  if (!object) return;
  recordHistory();
  if (reorderObject(scene(), object.id, direction)) render();
}
function toggleSelectedLock() {
  const object = selected();
  if (!object) return;
  recordHistory();
  object.locked = !object.locked;
  render();
}
function toggleSelectedVisibility() {
  const object = selected();
  if (!object) return;
  recordHistory();
  object.hidden = !object.hidden;
  render();
}
function pasteClipboard(inPlace=false){
  if(!state.clipboard){showToast("Clipboard is empty.");return;}
  recordHistory();
  const copy=structuredClone(state.clipboard);
  copy.id="object-"+Date.now().toString(36);
  if(!inPlace){copy.x=Math.min(viewport().width-copy.width,copy.x+24);copy.y=Math.min(viewport().height-copy.height,copy.y+24);}
  scene().objects.push(copy);
  state.selectedObjectId=copy.id;
  state.selectedObjectIds=[copy.id];
  render();
}

function setZoom(value){ state.zoom=Math.max(0.5,Math.min(2,Number(value)||1)); const pct=Math.round(state.zoom*100)+"%"; document.querySelectorAll(".zoom,.zoom-value").forEach(el=>el.textContent=pct); render(false); }
function fitSelection(){
  const objects=selectedObjects();
  if(!objects.length){fitCanvas();return;}
  const minX=Math.min(...objects.map(o=>o.x)), minY=Math.min(...objects.map(o=>o.y));
  const maxX=Math.max(...objects.map(o=>o.x+o.width)), maxY=Math.max(...objects.map(o=>o.y+o.height));
  const vp=viewport();
  state.pan={x:Math.max(-200,Math.min(200,(vp.width/2-(minX+maxX)/2))),y:Math.max(-160,Math.min(160,(vp.height/2-(minY+maxY)/2)))};
  render(false);
}
function fitCanvas() { state.zoom = 1; state.pan={x:0,y:0}; render(false); }
function selectedObjects() { return state.selectedObjectIds.map(id => scene().objects.find(object => object.id === id)).filter(Boolean); }
function alignSelected(mode) {
  const objects = selectedObjects();
  if (objects.length < 2) return;
  recordHistory();
  const vp = viewport();
  if (mode === "left") { const x = Math.min(...objects.map(o => o.x)); objects.forEach(o => moveObject(o, x, o.y, vp)); }
  if (mode === "right") { const right = Math.max(...objects.map(o => o.x + o.width)); objects.forEach(o => moveObject(o, right - o.width, o.y, vp)); }
  if (mode === "top") { const y = Math.min(...objects.map(o => o.y)); objects.forEach(o => moveObject(o, o.x, y, vp)); }
  if (mode === "bottom") { const bottom = Math.max(...objects.map(o => o.y + o.height)); objects.forEach(o => moveObject(o, o.x, bottom - o.height, vp)); }
  if (mode === "centerX") { const center = (Math.min(...objects.map(o => o.x)) + Math.max(...objects.map(o => o.x + o.width))) / 2; objects.forEach(o => moveObject(o, center - o.width / 2, o.y, vp)); }
  if (mode === "centerY") { const center = (Math.min(...objects.map(o => o.y)) + Math.max(...objects.map(o => o.y + o.height))) / 2; objects.forEach(o => moveObject(o, o.x, center - o.height / 2, vp)); }
  render();
}
function distributeSelected(axis) {
  const objects = selectedObjects();
  if (objects.length < 3) return;
  recordHistory();
  const ordered = [...objects].sort((a,b) => axis === "x" ? a.x-b.x : a.y-b.y);
  const first = ordered[0], last = ordered.at(-1);
  const spanStart = axis === "x" ? first.x : first.y;
  const spanEnd = axis === "x" ? (last.x + last.width) : (last.y + last.height);
  const totalSize = ordered.reduce((sum,o) => sum + (axis === "x" ? o.width : o.height), 0);
  const gap = (spanEnd - spanStart - totalSize) / (ordered.length - 1);
  let cursor = spanStart;
  const vp = viewport();
  for (const object of ordered) {
    if (axis === "x") moveObject(object, cursor, object.y, vp);
    else moveObject(object, object.x, cursor, vp);
    cursor += (axis === "x" ? object.width : object.height) + gap;
  }
  render();
}
function openLayersPanel() {
  let panel = document.querySelector("#syn-layers-panel");
  if (!panel) {
    panel = document.createElement("section");
    panel.id = "syn-layers-panel";
    panel.className = "modal";
    panel.innerHTML = '<div class="layers-browser"><div class="code-head"><strong>LAYERS</strong><button id="close-layers">Close</button></div><div id="layer-list"></div></div>';
    document.body.appendChild(panel);
    panel.querySelector("#close-layers").addEventListener("click", () => panel.hidden = true);
    panel.addEventListener("click", event => { if (event.target === panel) panel.hidden = true; });
  }
  const list = panel.querySelector("#layer-list");
  list.innerHTML = scene().objects.slice().reverse().map(object => '<button class="layer-row" data-layer="' + escapeHtml(object.id) + '"><span>' + escapeHtml(object.label) + '</span><small>' + (object.hidden ? "Hidden" : object.locked ? "Locked" : object.kind) + '</small></button>').join("") || '<div class="interaction empty">No objects in this scene.</div>';
  list.querySelectorAll(".layer-row").forEach(row => row.addEventListener("click", () => { state.selectedObjectId = row.dataset.layer; state.selectedObjectIds = [row.dataset.layer]; panel.hidden = true; render(); }));
  panel.hidden = false;
}


function render(keepSelection = true) {
  stage.querySelectorAll(".syn-object,.resize-handle").forEach(el => el.remove());
  const activeScene = scene();
  const logical = viewport();
  const fitScale = responsiveScale(logical.width, logical.height, Math.max(1, stage.clientWidth - 2), Math.max(1, stage.clientHeight - 2));
  const mobile = window.innerWidth <= 560;
  // Phones need an authoring canvas, not a microscopic desktop screenshot. Keep a readable
  // minimum scale and let the artboard pan/scroll instead of shrinking the work to illegible text.
  const scale = (mobile ? Math.max(0.64, fitScale) : fitScale) * state.zoom;
  stage.classList.toggle("mobile-canvas", mobile);
  stage.style.backgroundPosition = state.pan.x+"px "+state.pan.y+"px";
  emptyState.hidden = activeScene.objects.some(object => !object.hidden);
  sceneLabel.textContent = activeScene.name;
  document.querySelector("#prevScene").disabled = state.sceneIndex === 0;
  document.querySelector("#nextScene").disabled = state.sceneIndex === state.document.scenes.length - 1;
  document.querySelectorAll(".zoom,.zoom-value").forEach(el => el.textContent = Math.round(state.zoom * 100) + "%");
  const objectCount = activeScene.objects.filter(object => !object.hidden).length;
  const canvasObjectCount = document.querySelector("#canvasObjectCount");
  const canvasSceneCount = document.querySelector("#canvasSceneCount");
  const canvasStatus = document.querySelector("#canvasStatus");
  const studioStatusText = document.querySelector("#studioStatusText");
  const canvasModeLabel = document.querySelector("#canvasModeLabel");
  const canvasHint = document.querySelector("#canvasHint");
  if (canvasObjectCount) canvasObjectCount.textContent = objectCount + (objectCount === 1 ? " object" : " objects");
  if (canvasSceneCount) canvasSceneCount.textContent = state.document.scenes.length + (state.document.scenes.length === 1 ? " scene" : " scenes");
  if (canvasStatus) canvasStatus.textContent = state.selectedObjectIds.length ? state.selectedObjectIds.length + " selected" : "Ready";
  if (studioStatusText) studioStatusText.textContent = state.selectedObjectIds.length ? "Editing " + (selected()?.label || "object") : "Ready";
  if (canvasModeLabel) canvasModeLabel.textContent = state.workspaceMode.toUpperCase();
  if (canvasHint) canvasHint.textContent = state.selectedObjectIds.length ? "Inspector is open" : "Select an object to edit it";
  for (const object of activeScene.objects) {
    const el = document.createElement("button");
    el.className = "syn-object syn-" + object.kind + (state.selectedObjectIds.includes(object.id) ? " selected" : "");
    el.dataset.id = object.id;
    el.dataset.kind = object.kind;
    el.type = "button";
    el.disabled = Boolean(object.locked);
    el.hidden = Boolean(object.hidden);
    el.setAttribute("aria-label", object.label || object.kind);
    el.setAttribute("aria-selected", state.selectedObjectIds.includes(object.id) ? "true" : "false");
    el.style.left = Math.round(object.x * scale + state.pan.x) + "px";
    el.style.top = Math.round(object.y * scale + state.pan.y) + "px";
    el.style.width = Math.max(20, Math.round(object.width * scale)) + "px";
    el.style.height = Math.max(20, Math.round(object.height * scale)) + "px";
    el.style.transform = "rotate(" + Number(object.rotation || 0) + "deg)";
    el.textContent = object.props?.text || object.label;
    const supported = {
      fontFamily:"fontFamily",fontSize:"fontSize",fontWeight:"fontWeight",fontStyle:"fontStyle",color:"color",
      background:"background",borderRadius:"borderRadius",borderColor:"borderColor",borderWidth:"borderWidth",
      boxShadow:"boxShadow",letterSpacing:"letterSpacing",lineHeight:"lineHeight",opacity:"opacity",
      textAlign:"textAlign",padding:"padding",textTransform:"textTransform"
    };
    for (const [key,value] of Object.entries(object.styles || {})) {
      const prop = supported[key];
      if (!prop || value == null) continue;
      el.style[prop] = typeof value === "number" && ["fontSize","borderRadius","borderWidth"].includes(key) ? value * (key === "fontSize" ? scale : 1) + "px" : value;
    }
    if (object.props?.src?.startsWith("data:image/")) {
      el.style.backgroundImage = 'url("' + object.props.src + '")';
      el.style.backgroundSize = "cover";
      el.style.backgroundPosition = "center";
      el.textContent = "";
    }
    if (object.kind==="audio") {
      el.textContent = "♫  " + (object.label || "Audio");
    }
    if (object.kind==="video") {
      el.textContent = "▶  " + (object.label || "Video");
    }
    el.addEventListener("click", event => {
      event.stopPropagation();
      selectObject(object.id, event.shiftKey);
    });
    el.addEventListener("dblclick", event => {
      if (object.kind !== "text" || object.locked) return;
      event.stopPropagation();
      beginInlineEdit(object, scale);
    });
    if (!object.locked && !object.hidden && state.toolMode==="move") el.addEventListener("pointerdown", event => beginDrag(event, object, scale));
    stage.appendChild(el);
    if (state.selectedObjectIds.includes(object.id) && !object.locked && !object.hidden) {
      for (const direction of ["nw","ne","sw","se"]) {
        const handle = document.createElement("div");
        handle.className = "resize-handle resize-" + direction;
        const hx = direction.includes("e") ? (object.x + object.width) : object.x;
        const hy = direction.includes("s") ? (object.y + object.height) : object.y;
        handle.style.left = Math.round(hx * scale + state.pan.x) + "px";
        handle.style.top = Math.round(hy * scale + state.pan.y) + "px";
        handle.dataset.id = object.id;
        handle.dataset.direction = direction;
        handle.setAttribute("aria-label", "Resize " + object.label);
        handle.addEventListener("pointerdown", event => beginResize(event, object, direction, scale));
        stage.appendChild(handle);
      }
    }
  }
  renderInteractions();
  if (keepSelection && state.selectedObjectId && selected()) selectObject(state.selectedObjectId);
}
function beginInlineEdit(object, scale) {
  const input = document.createElement("textarea");
  input.className = "inline-edit";
  input.value = object.props?.text ?? object.label;
  input.style.left = Math.round(object.x * scale) + "px";
  input.style.top = Math.round(object.y * scale) + "px";
  input.style.width = Math.round(object.width * scale) + "px";
  input.style.height = Math.round(object.height * scale) + "px";
  input.style.fontFamily = object.styles?.fontFamily || "Inter";
  input.style.fontSize = Math.max(8, (object.styles?.fontSize || 24) * scale) + "px";
  stage.appendChild(input);
  input.focus();
  input.select();
  let committed = false;
  const commit = () => {
    if (committed) return;
    committed = true;
    recordHistory();
    object.props = { ...object.props, text: input.value };
    input.remove();
    render();
  };
  input.addEventListener("blur", commit, { once: true });
  input.addEventListener("keydown", event => {
    if (event.key === "Escape") { committed = true; input.remove(); render(); }
    if (event.key === "Enter" && !event.shiftKey) { event.preventDefault(); commit(); }
  });
}
function beginDrag(event, object, scale) {
  if (event.button !== 0 || object.locked || state.toolMode!=="move") return;
  event.preventDefault();
  const startX = event.clientX, startY = event.clientY;
  const originX = object.x, originY = object.y;
  let changed = false;
  selectObject(object.id, event.shiftKey);
  const move = e => {
    const dx = (e.clientX - startX) / Math.max(scale, 0.01);
    const dy = (e.clientY - startY) / Math.max(scale, 0.01);
    if (Math.abs(dx) + Math.abs(dy) > 1 && !changed) { recordHistory(); changed = true; }
    if (!changed) return;
    const snap = state.snap ? 8 : 1;
    const nextX = Math.round((originX + dx) / snap) * snap;
    const nextY = Math.round((originY + dy) / snap) * snap;
    moveObject(object, nextX, nextY, viewport());
    render(false);
  };
  const end = () => {
    window.removeEventListener("pointermove", move);
    window.removeEventListener("pointerup", end);
    if (changed) render();
  };
  window.addEventListener("pointermove", move);
  window.addEventListener("pointerup", end, { once: true });
}
function beginResize(event, object, direction, scale) {
  if (event.button !== 0 || object.locked) return;
  event.preventDefault();
  event.stopPropagation();
  const startX = event.clientX, startY = event.clientY;
  const origin = { x: object.x, y: object.y, width: object.width, height: object.height };
  let changed = false;
  const move = e => {
    const dx = (e.clientX - startX) / Math.max(scale, 0.01);
    const dy = (e.clientY - startY) / Math.max(scale, 0.01);
    let x = origin.x, y = origin.y, width = origin.width, height = origin.height;
    if (direction.includes("e")) width = origin.width + dx;
    if (direction.includes("s")) height = origin.height + dy;
    if (direction.includes("w")) { width = origin.width - dx; x = origin.x + dx; }
    if (direction.includes("n")) { height = origin.height - dy; y = origin.y + dy; }
    const min = 20;
    if (width < min) { if (direction.includes("w")) x = origin.x + origin.width - min; width = min; }
    if (height < min) { if (direction.includes("n")) y = origin.y + origin.height - min; height = min; }
    const vp = viewport();
    if (x < 0) { width += x; x = 0; }
    if (y < 0) { height += y; y = 0; }
    if (x + width > vp.width) width = vp.width - x;
    if (y + height > vp.height) height = vp.height - y;
    if (!changed) { recordHistory(); changed = true; }
    object.x = Math.round(x); object.y = Math.round(y);
    resizeObject(object, width, height, vp, min);
    render(false);
  };
  const end = () => {
    window.removeEventListener("pointermove", move);
    window.removeEventListener("pointerup", end);
    if (changed) render();
  };
  window.addEventListener("pointermove", move);
  window.addEventListener("pointerup", end, { once: true });
}

function preview() {
  setWorkspaceMode("design");
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
  return `body{margin:0;background:#090a0d;color:#fff;font-family:system-ui}header{height:56px;padding:0 18px;display:flex;align-items:center;justify-content:space-between;border-bottom:1px solid #242833}main{position:relative;margin:20px;min-height:600px;border:1px solid #2b2f39;border-radius:12px;background:#0c0e13;background-image:linear-gradient(#151821 1px,transparent 1px),linear-gradient(90deg,#151821 1px,transparent 1px);background-size:24px 24px}.syn-runtime-object{position:absolute;border:1px solid #3a4050;border-radius:8px;background:#171a22;color:#fff;padding:10px 14px;white-space:pre-wrap;overflow:hidden}.syn-runtime-object[data-kind=button]{background:#78a9ff;color:#160b14;border-color:transparent;font-weight:800;cursor:pointer;text-align:center}`;
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

function openAssetBrowser() {
  let panel = document.querySelector("#syn-assets-panel");
  if (!panel) {
    panel = document.createElement("section");
    panel.id = "syn-assets-panel";
    panel.className = "modal";
    panel.innerHTML = '<div class="asset-browser"><div class="code-head"><strong>ASSETS</strong><button id="close-assets">Close</button></div><div id="asset-grid" class="asset-grid"></div></div>';
    document.body.appendChild(panel);
    panel.querySelector("#close-assets").addEventListener("click", () => panel.hidden = true);
    panel.addEventListener("click", event => { if (event.target === panel) panel.hidden = true; });
  }
  const grid = panel.querySelector("#asset-grid");
  grid.innerHTML = state.document.assets.length ? state.document.assets.map(asset => '<div class="asset-card"><strong>' + escapeHtml(asset.name) + '</strong><small>' + escapeHtml(asset.type || "unknown") + '</small><small>' + Math.max(1, Math.round((asset.size || 0) / 1024)) + ' KB</small></div>').join("") : '<div class="interaction empty">No embedded assets yet.</div>';
  panel.hidden = false;
}
document.querySelector("#mediaInput").addEventListener("change", async event => {
  const files=[...(event.target.files||[])].filter(file=>/^(image|audio|video)\//.test(file.type));
  if(!files.length){event.target.value="";return;}
  for(const file of files){
    const reader=new FileReader();
    await new Promise(resolve=>{
      reader.onload=()=>{
        recordHistory();
        const asset={id:"asset-"+Date.now().toString(36)+"-"+Math.random().toString(36).slice(2,7),name:file.name,type:file.type,size:file.size,embedded:true};
        state.document.assets.push(asset);
        const count=scene().objects.length;
        const kind=file.type.startsWith("audio/")?"audio":file.type.startsWith("video/")?"video":"media";
        const object=addObject(scene(),{
          kind,
          label:file.name.replace(/\.[^.]+$/,"")||"Media",
          x:60+(count%4)*40,
          y:60+(count%4)*40,
          width:220,
          height:160,
          props:{src:String(reader.result),assetId:asset.id,alt:file.name.replace(/\.[^.]+$/,"")||"Media",mediaType:file.type}
        });
        state.selectedObjectId=object.id;
        state.selectedObjectIds=[object.id];
        resolve();
      };
      reader.readAsDataURL(file);
    });
  }
  render();
  event.target.value="";
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
stage.addEventListener("click", () => { if(state.toolMode!=="pan") selectObject(null); });
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
  state.selectedObjectIds = [];
  state.history = createHistory(state.document, { limit: 100 });
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
    state.selectedObjectIds = [];
    state.history = createHistory(state.document, { limit: 100 });
    render();
  } catch (error) {
    alert(error instanceof Error ? error.message : "Unable to open SYN document.");
  }
  event.target.value = "";
});

document.querySelector("#preview").addEventListener("click", preview);
document.querySelector("#toolbarPreview")?.addEventListener("click",preview);
document.querySelector("#toolbarFit")?.addEventListener("click",fitCanvas);
document.querySelector("#toolbarZoomOut")?.addEventListener("click",()=>setZoom(state.zoom-0.1));
document.querySelector("#toolbarZoomIn")?.addEventListener("click",()=>setZoom(state.zoom+0.1));
document.querySelector("#toolbarZoomValue")?.addEventListener("click",()=>setZoom(1));
document.querySelector("#toolbarCommand")?.addEventListener("click",openCodePalette);
document.querySelector("#emptyText")?.addEventListener("click",()=>addCanvasObject("text"));
document.querySelector("#emptyShape")?.addEventListener("click",()=>addCanvasObject("shape"));
document.querySelector("#emptyMedia")?.addEventListener("click",()=>document.querySelector("#mediaInput")?.click());
document.querySelector("#emptyCommands")?.addEventListener("click",openCodePalette);
stage.addEventListener("keydown",event=>{
  if(event.target!==stage || event.metaKey || event.ctrlKey || event.altKey) return;
  if(event.key===" "){event.preventDefault();state.toolMode=state.toolMode==="pan"?"select":"pan";document.querySelectorAll("[data-studio-tool]").forEach(item=>item.classList.toggle("active",item.dataset.studioTool===state.toolMode));stage.classList.toggle("pan-mode",state.toolMode==="pan");}
});
document.querySelectorAll("[data-workspace-mode]").forEach(button=>button.addEventListener("click",()=>setWorkspaceMode(button.dataset.workspaceMode)));
document.querySelectorAll("[data-studio-tool]").forEach(button=>button.addEventListener("click",()=>{
  const tool=button.dataset.studioTool;
  state.toolMode=tool;
  document.querySelectorAll("[data-studio-tool]").forEach(item=>item.classList.toggle("active",item.dataset.studioTool===tool));
  if(["text","shape","button","media"].includes(tool)) addCanvasObject(tool);
  stage.classList.toggle("pan-mode",tool==="pan");
}));
stage.addEventListener("pointerdown",event=>{
  if(state.toolMode!=="pan" || event.target.closest(".syn-object,.resize-handle"))return;
  event.preventDefault();
  const startX=event.clientX,startY=event.clientY,origin={...state.pan};
  const move=next=>{
    state.pan={x:origin.x+(next.clientX-startX),y:origin.y+(next.clientY-startY)};
    render(false);
  };
  const end=()=>{
    window.removeEventListener("pointermove",move);
    window.removeEventListener("pointerup",end);
  };
  window.addEventListener("pointermove",move);
  window.addEventListener("pointerup",end,{once:true});
});
window.addEventListener("resize", () => render(false));
document.addEventListener("keydown", event => {
  const mod = event.metaKey || event.ctrlKey;
  const target = event.target;
  if (mod && !event.shiftKey && event.key.toLowerCase() === "z") { event.preventDefault(); undo(); return; }
  if (mod && event.shiftKey && event.key.toLowerCase() === "z") { event.preventDefault(); redo(); return; }
  if (mod && event.key.toLowerCase() === "d" && !target.matches("input,textarea,select")) { event.preventDefault(); duplicateSelected(); return; }
  if (mod && event.key === "Enter" && !target.matches("input,textarea,select")) { event.preventDefault(); fitCanvas(); return; }
  if (!target.matches("input,textarea,select")) {
    const key=event.key.toLowerCase();
    const shortcuts={v:"select",m:"move",h:"pan",t:"text",r:"shape"};
    if(shortcuts[key] && !mod){
      event.preventDefault();
      state.toolMode=shortcuts[key];
      document.querySelectorAll("[data-studio-tool]").forEach(item=>item.classList.toggle("active",item.dataset.studioTool===state.toolMode));
      stage.classList.toggle("pan-mode",state.toolMode==="pan");
      if(["text","shape"].includes(state.toolMode)) addCanvasObject(state.toolMode);
      return;
    }
    if(event.key==="Enter" && selected()?.kind==="text" && !event.shiftKey){
      event.preventDefault();
      const logical=viewport();
      const scale=responsiveScale(logical.width,logical.height,Math.max(1,stage.clientWidth-2),Math.max(1,stage.clientHeight-2))*state.zoom;
      beginInlineEdit(selected(),scale);
      return;
    }
    if(event.key==="F2" && selected()){ event.preventDefault(); renameSelectedObject(); return; }
    if(event.key==="Delete" || event.key==="Backspace"){ if(selected()){ event.preventDefault(); deleteSelectedObject(); return; } }
  }
  if (!target.matches("input,textarea,select") && ["ArrowUp","ArrowDown","ArrowLeft","ArrowRight"].includes(event.key) && selected()) {
    event.preventDefault();
    recordHistory();
    const step = event.shiftKey ? 10 : 1;
    const dx = event.key === "ArrowRight" ? step : event.key === "ArrowLeft" ? -step : 0;
    const dy = event.key === "ArrowDown" ? step : event.key === "ArrowUp" ? -step : 0;
    selectedObjects().forEach(object => moveObject(object, object.x + dx, object.y + dy, viewport()));
    render();
  }
});
buildApplicationMenus();
const inspectorToggle=document.querySelector("#inspectorToggle");
inspectorToggle?.addEventListener("click",()=>{
  document.body.classList.toggle("show-inspector");
  inspectorToggle.setAttribute("aria-expanded",String(document.body.classList.contains("show-inspector")));
});
document.addEventListener("click",event=>{
  if(document.body.classList.contains("show-inspector") && !event.target.closest(".inspector,.mobile-inspector-toggle")){
    document.body.classList.remove("show-inspector");
    inspectorToggle?.setAttribute("aria-expanded","false");
  }
});
document.addEventListener("keydown",e=>{
  if(e.key==="Escape" && document.body.classList.contains("show-inspector")){
    document.body.classList.remove("show-inspector");
    inspectorToggle?.setAttribute("aria-expanded","false");
  }
  if((e.metaKey||e.ctrlKey)&&e.key.toLowerCase()==="k"){e.preventDefault();openCodePalette()}
});
state.history = createHistory(state.document, { limit: 100 });
document.querySelector("#prevScene").addEventListener("click", () => changeScene(-1));
document.querySelector("#nextScene").addEventListener("click", () => changeScene(1));
render();
