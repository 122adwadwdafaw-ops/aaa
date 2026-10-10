// Original adapters for documented ExtendScript serialization behavior.
// Cyclic/host object serialization is intentionally rejected; it is not JSON.
// Host type tags are distinct from File.toString(), which returns its path.
Object.defineProperty(File.prototype,Symbol.toStringTag,{value:'File'});
Object.defineProperty(Folder.prototype,Symbol.toStringTag,{value:'Folder'});
function __ecToSource(value, depth, seen, budget) {
  budget=budget||{nodes:0,bytes:0};
  if(++budget.nodes>100000)throw new RangeError('toSource node limit exceeded');
  var result=__ecToSourceValue(value,depth,seen,budget);
  budget.bytes+=result.length;
  if(budget.bytes>32*1024*1024)throw new RangeError('toSource output budget exceeded');
  return result;
}
function __ecToSourceValue(value, depth, seen, budget) {
  if(depth>64)throw new RangeError('toSource nesting limit exceeded');
  if(value===null)return 'null';
  if(value===undefined)return 'undefined';
  if(typeof value==='string')return JSON.stringify(value);
  if(typeof value==='number')return Object.is(value,-0)?'-0':String(value);
  if(typeof value==='boolean')return String(value);
  if(typeof value==='function')return '('+Function.prototype.toString.call(value)+')';
  if(value instanceof String || value instanceof Number || value instanceof Boolean)return __ecToSource(value.valueOf(),depth+1,seen,budget);
  if(value instanceof Date)return '(new Date('+String(value.getTime())+'))';
  if(value instanceof RegExp)return String(value);
  if(value instanceof File || value instanceof Folder)return '(new '+(value instanceof File?'File':'Folder')+'('+JSON.stringify(value.fsName)+'))';
  if(seen.indexOf(value)>=0)throw new TypeError('toSource does not support cyclic objects');
  seen.push(value);
  var result;
  if(Array.isArray(value)) {
    if(value.length>100000)throw new RangeError('toSource array limit exceeded');
    var items=[];
    for(var i=0;i<value.length;i++)items.push(i in value?__ecToSource(value[i],depth+1,seen,budget):'');
    result='['+items.join(',')+(value.length && !(value.length-1 in value)?',':'')+']';
  } else {
    var keys=Object.keys(value);
    if(keys.length>100000)throw new RangeError('toSource object limit exceeded');
    result='({'+keys.map(function(key){return JSON.stringify(key)+':'+__ecToSource(value[key],depth+1,seen,budget);}).join(',')+'})';
  }
  seen.pop();
  if(result.length>16*1024*1024)throw new RangeError('toSource output limit exceeded');
  return result;
}
Object.defineProperty(Object.prototype,'toSource',{value:function(){return __ecToSource(this,0,[]);},writable:true,configurable:true});
Object.defineProperty(Function.prototype,'toSource',{value:function(){return '('+Function.prototype.toString.call(this)+')';},writable:true,configurable:true});
