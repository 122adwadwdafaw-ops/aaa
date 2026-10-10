var __jsxbinRegExp = RegExp;
// Original, bounded XML value adapter for the JSXBIN loader. This implements the
// value operations exercised by the synthetic corpus, not all ECMA-357 behavior.
var __jsxbinDefaultNamespace = "";
function Namespace(prefix, uri) {
  if (!(this instanceof Namespace)) return new Namespace(prefix, uri);
  this.prefix = uri === undefined ? "" : String(prefix);
  this.uri = uri === undefined ? (prefix === undefined ? "" : String(prefix)) : String(uri);
}
Namespace.prototype.toString = function () { return this.uri; };
function QName(ns, name) {
  if (!(this instanceof QName)) return new QName(ns, name);
  this.uri = name === undefined ? __jsxbinDefaultNamespace : ns === null ? null : String(ns);
  this.localName = String(name === undefined ? ns : name);
}
QName.prototype.toString = function () { return this.uri ? this.uri + "::" + this.localName : this.localName; };
function setDefaultXMLNamespace(ns) { __jsxbinDefaultNamespace = String(ns); }
var __jsxbinXmlNodes = new WeakMap(), __jsxbinXmlParents = new WeakMap();
function __jsxbinEscapeXML(s, attr) {
  s = String(s).replace(/&/g,"&amp;").replace(/</g,"&lt;").replace(/>/g,"&gt;");
  return attr ? s.replace(/"/g,"&quot;").replace(/\n/g,"&#xA;").replace(/\r/g,"&#xD;").replace(/\t/g,"&#x9;") : s;
}
function __jsxbinXmlPart(value, mode) {
  if (mode === 0) return String(value);
  if (mode === 1) {
    var name = String(value);
    if (!/^[A-Za-z_][A-Za-z0-9_.:\-]*$/.test(name)) throw new SyntaxError("Invalid XML element name");
    return name;
  }
  if (mode === 2) return '"' + __jsxbinEscapeXML(value,true) + '"';
  return __jsxbinXmlNodes.has(value) ? value.toXMLString() : __jsxbinEscapeXML(value,false);
}
function __jsxbinXml(source) { return /^\s*<>/.test(String(source)) ? XMLList(source) : XML(source); }
function __jsxbinXmlRead(source) {
  var nodes = JSON.parse(__xmlParse(String(source),__jsxbinDefaultNamespace));
  function clean(list, parent, depth) {
    if (depth > 96) throw new RangeError("XML nesting limit exceeded");
    return list.filter(function(n) {
      if (XML.ignoreComments && n.kind === "comment") return false;
      if (XML.ignoreProcessingInstructions && n.kind === "processing-instruction") return false;
      if (XML.ignoreWhitespace && n.kind === "text" && !n.value.trim()) return false;
      if (parent) __jsxbinXmlParents.set(n,parent);
      if (n.children) n.children = clean(n.children,n,depth+1);
      if (n.attributes) n.attributes.forEach(function(a){__jsxbinXmlParents.set(a,n);});
      return true;
    });
  }
  return clean(nodes,null,0);
}
function XML(source) {
  if (__jsxbinXmlNodes.has(source)) return source.copy();
  source = source === undefined || source === null ? "" : String(source);
  var nodes = source.indexOf("<") < 0 ? [{kind:"text",value:source}] : __jsxbinXmlRead(source);
  if (nodes.length !== 1) throw new TypeError("XML requires exactly one node; use XMLList for multiple nodes");
  return __jsxbinXmlWrap(nodes);
}
function XMLList(source) {
  if (__jsxbinXmlNodes.has(source)) return __jsxbinXmlWrap(__jsxbinXmlNodes.get(source).slice());
  return __jsxbinXmlWrap(source === undefined || source === null || source === "" ? [] : __jsxbinXmlRead(source));
}
XML.ignoreComments = true; XML.ignoreProcessingInstructions = true; XML.ignoreWhitespace = true;
XML.prettyPrinting = true; XML.prettyIndent = 2;
XML.settings = function(){ return {ignoreComments:XML.ignoreComments,ignoreProcessingInstructions:XML.ignoreProcessingInstructions,ignoreWhitespace:XML.ignoreWhitespace,prettyPrinting:XML.prettyPrinting,prettyIndent:XML.prettyIndent}; };
XML.defaultSettings = function(){ return {ignoreComments:true,ignoreProcessingInstructions:true,ignoreWhitespace:true,prettyPrinting:true,prettyIndent:2}; };
XML.setSettings = function(s){s=s||XML.defaultSettings();Object.keys(XML.defaultSettings()).forEach(function(k){if(k in s)XML[k]=s[k];});};
function __jsxbinXmlSerialize(n,depth,namespace) {
  if (depth > 96) throw new RangeError("XML nesting limit exceeded");
  if(n.kind === "text" || n.kind === "attribute") return __jsxbinEscapeXML(n.value,n.kind === "attribute");
  if(n.kind === "comment") return "<!--"+n.value+"-->";
  if(n.kind === "processing-instruction") return "<?"+n.name+(n.value?" "+n.value:"")+"?>";
  var name=(n.prefix?n.prefix+":":"")+n.name;
  var ns=Object.assign({},namespace||{}), attrs="";
  function declare(prefix,uri){ if(ns[prefix]!==uri){attrs+=' xmlns'+(prefix?":"+prefix:"")+'="'+__jsxbinEscapeXML(uri,true)+'"';ns[prefix]=uri;} }
  if(n.ns || ns[n.prefix||""]) declare(n.prefix||"",n.ns||"");
  (n.attributes||[]).forEach(function(a){if(a.ns && a.prefix)declare(a.prefix,a.ns);attrs+=' '+(a.prefix?a.prefix+":":"")+a.name+'="'+__jsxbinEscapeXML(a.value,true)+'"';});
  if(!n.children.length)return "<"+name+attrs+"/>";
  var pretty=XML.prettyPrinting && n.children.every(function(c){return c.kind!=="text";});
  var width=Math.max(0,Math.min(16,Number(XML.prettyIndent)||0));
  var text=n.children.map(function(c){return (pretty?" ".repeat((depth+1)*width):"")+__jsxbinXmlSerialize(c,depth+1,ns);}).join(pretty?"\n":"");
  return "<"+name+attrs+">"+(pretty?"\n":"")+text+(pretty?"\n"+" ".repeat(depth*width):"")+"</"+name+">";
}
function __jsxbinXmlSelect(nodes,key,attr,desc,ns) {
  var result=[];
  if(key instanceof QName){ns=key.uri;key=key.localName;}
  key=String(key);
  if(key.charAt(0)==="@"){attr=true;key=key.slice(1);}
  function visit(n,depth){
    if(depth>96)throw new RangeError("XML nesting limit exceeded");
    (attr?n.attributes||[]:n.children||[]).forEach(function(c){
      if((attr||c.kind==="element")&&(key==="*"||c.name===key)&&(ns===null||ns===undefined||String(ns)===c.ns))result.push(c);
    });
    if(desc)(n.children||[]).forEach(function(c){visit(c,depth+1);});
  }
  nodes.forEach(function(n){visit(n,0);});return result;
}
function __jsxbinXmlWrap(nodes) {
  var methods={
    length:function(){return nodes.length;},
    toXMLString:function(){return nodes.map(function(n){return __jsxbinXmlSerialize(n,0,{});}).join(XML.prettyPrinting?"\n":"");},
    hasSimpleContent:function(){return nodes.every(function(n){return !(n.children||[]).some(function(c){return c.kind==="element";});});},
    hasComplexContent:function(){return !methods.hasSimpleContent();},
    toString:function(){return methods.hasSimpleContent()?nodes.map(function(n){return n.kind==="element"?(n.children||[]).filter(function(c){return c.kind==="text";}).map(function(c){return c.value;}).join(""):n.value||"";}).join(""):methods.toXMLString();},
    valueOf:function(){return proxy;},
    copy:function(){return __jsxbinXmlWrap(JSON.parse(JSON.stringify(nodes)));},
    child:function(k){return /^\d+$/.test(String(k))?__jsxbinXmlWrap(nodes.flatMap(function(n){var c=(n.children||[])[Number(k)];return c?[c]:[];})):__jsxbinXmlWrap(__jsxbinXmlSelect(nodes,k,false,false));},
    children:function(){return __jsxbinXmlWrap(nodes.flatMap(function(n){return n.children||[];}));},
    elements:function(k){return methods.child(k===undefined?"*":k);},
    attributes:function(){return methods.attribute("*");},
    attribute:function(k){return __jsxbinXmlWrap(__jsxbinXmlSelect(nodes,k,true,false));},
    descendants:function(k){return __jsxbinXmlWrap(__jsxbinXmlSelect(nodes,k===undefined?"*":k,false,true));},
    text:function(){return __jsxbinXmlWrap(nodes.flatMap(function(n){return (n.children||[]).filter(function(c){return c.kind==="text";});}));},
    name:function(){var n=nodes[0];return n&&n.name?new QName(n.ns||"",n.name):null;},
    localName:function(){var n=nodes[0];return n&&n.name?n.name:null;},
    nodeKind:function(){return nodes[0]?nodes[0].kind:undefined;},
    namespace:function(prefix){var n=nodes[0];if(!n)return undefined;if(prefix===undefined)return new Namespace(n.prefix||"",n.ns||"");var ns=(n.namespaces||[]).find(function(s){return s.prefix===String(prefix);});return ns?new Namespace(ns.prefix,ns.uri):undefined;},
    namespaceDeclarations:function(){return ((nodes[0]||{}).namespaces||[]).map(function(n){return new Namespace(n.prefix,n.uri);});},
    inScopeNamespaces:function(){return methods.namespaceDeclarations();},
    parent:function(){var ps=nodes.map(function(n){return __jsxbinXmlParents.get(n);});return ps.length&&ps[0]&&ps.every(function(n){return n===ps[0];})?__jsxbinXmlWrap([ps[0]]):undefined;},
    contains:function(other){return nodes.some(function(n){return __jsxbinXmlSerialize(n,0,{})===String(other.toXMLString?other.toXMLString():other);});},
    appendChild:function(value){return mutateChildren(value,false);},
    prependChild:function(value){return mutateChildren(value,true);},
    setChildren:function(value){nodes.forEach(function(n){if(n.children)n.children=[];});return mutateChildren(value,false);},
    setName:function(name){nodes.forEach(function(n){if(n.kind==="element"||n.kind==="attribute")n.name=String(name.localName||name);});return undefined;},
    setLocalName:function(name){return methods.setName(name);},
    setNamespace:function(ns){nodes.forEach(function(n){n.ns=String(ns);n.prefix=ns.prefix||"";});return undefined;},
  };
  function copies(value){return __jsxbinXmlNodes.has(value)?JSON.parse(JSON.stringify(__jsxbinXmlNodes.get(value))):[{kind:"text",value:String(value)}];}
  function mutateChildren(value,prepend){nodes.forEach(function(n){if(n.kind!=="element")throw new TypeError("XML element required");var c=copies(value);c.forEach(function(x){__jsxbinXmlParents.set(x,n);});n.children=prepend?c.concat(n.children):n.children.concat(c);});return proxy;}
  var proxy=new Proxy({}, {
    get:function(_,key){
      if(key===Symbol.toPrimitive)return function(){return methods.toString();};
      if(key===Symbol.iterator)return function*(){for(var n of nodes)yield __jsxbinXmlWrap([n]);};
      if(typeof key!=="string")return undefined;
      if(key in methods)return methods[key];
      if(/^\d+$/.test(key))return nodes[Number(key)]?__jsxbinXmlWrap([nodes[Number(key)]]):undefined;
      if(methods.hasSimpleContent() && typeof String.prototype[key]==="function")return function(){return String.prototype[key].apply(methods.toString(),arguments);};
      return __jsxbinXmlWrap(__jsxbinXmlSelect(nodes,key,false,false));
    },
    has:function(_,key){return typeof key==="string"&&(key in methods||__jsxbinXmlSelect(nodes,key,false,false).length>0);},
    ownKeys:function(){return nodes.map(function(_,i){return String(i);});},
    getOwnPropertyDescriptor:function(_,key){if(/^\d+$/.test(String(key))&&Number(key)<nodes.length)return {configurable:true,enumerable:true,writable:true,value:__jsxbinXmlWrap([nodes[Number(key)]])};},
    set:function(_,key,value){
      if(typeof key!=="string")return false;
      var attr=key.charAt(0)==="@",name=attr?key.slice(1):key;
      nodes.forEach(function(n){
        if(n.kind!=="element")throw new TypeError("XML element required");
        if(attr){var a=n.attributes.find(function(a){return a.name===name;});if(a)a.value=String(value);else{a={kind:"attribute",name:name,ns:"",prefix:"",value:String(value)};n.attributes.push(a);__jsxbinXmlParents.set(a,n);}}
        else{var index=n.children.findIndex(function(c){return c.kind==="element"&&c.name===name;});n.children=n.children.filter(function(c){return c.kind!=="element"||c.name!==name;});var c=__jsxbinXmlNodes.has(value)?copies(value):[{kind:"element",name:name,ns:__jsxbinDefaultNamespace,prefix:"",attributes:[],namespaces:[],children:copies(value)}];c.forEach(function(x){__jsxbinXmlParents.set(x,n);});n.children.splice(index<0?n.children.length:index,0,...c);}
      });return true;
    },
    deleteProperty:function(_,key){var attr=String(key).charAt(0)==="@",name=attr?String(key).slice(1):String(key);nodes.forEach(function(n){var k=attr?"attributes":"children";n[k]=(n[k]||[]).filter(function(c){return name!=="*"&&c.name!==name;});});return true;}
  });
  __jsxbinXmlNodes.set(proxy,nodes);return proxy;
}
function __jsxbinXmlAccess(obj,key,second,third,attr,desc,ns){
  if(second!==null)ns=second;
  if(third!==null)key=third;
  if(!__jsxbinXmlNodes.has(obj))throw new TypeError("XML value required");
  return __jsxbinXmlWrap(__jsxbinXmlSelect(__jsxbinXmlNodes.get(obj),key,attr,desc,ns));
}
function __jsxbinXmlFilter(obj,predicate){
  if(!__jsxbinXmlNodes.has(obj))throw new TypeError("XML value required");
  return __jsxbinXmlWrap(__jsxbinXmlNodes.get(obj).filter(function(n){return predicate.call(__jsxbinXmlWrap([n]));}));
}
function __jsxbinXmlUnary(op,value){throw new Error("Unsupported XML unary operation: "+op);}
function __jsxbinAdd(a,b){return __jsxbinXmlNodes.has(a)&&__jsxbinXmlNodes.has(b)?__jsxbinXmlWrap(__jsxbinXmlNodes.get(a).concat(__jsxbinXmlNodes.get(b))):a+b;}
function __jsxbinTypeof(x){return __jsxbinXmlNodes.has(x)?"xml":typeof x;}
