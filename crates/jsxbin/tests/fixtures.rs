// Original synthetic scripts by @cdinhoo, MIT OR Apache-2.0.
// Compiled using the public Adobe ExtendScript Debugger 2.1.0 CLI as a black box.
// No third-party scripts or test data.
pub const CASES: &[(&str, &str, &str)] = &[
    (
        "original-fixtures_arithmetic",
        r####"(2 + 3) * 7 - 4 / 2;
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ABJAnACzBhNBnFdCdhDn0DzACByB"####,
    ),
    (
        "original-fixtures_array_holes",
        r####"var a=[1,,3]; a.length + ":" + (1 in a);
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ACJAnASzBjBByBARDFdBnFdDfnftJAnACzBhLCCCXzGjMjFjOjHjUjIDf
VBfyBnneBhaCzCjJjOEnVBfyBdBnnnABB40BiAABAzAFByB"####,
    ),
    (
        "original-fixtures_closure",
        r####"function f(x){return function(y){return x+y;};} f(6)(7);
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBnABMAbyBn0ABZAnANyBnAMAbyBn0ABZAnACzBhLBjzBjYCfVzBjZDfAnnAB
D40BhAB0AzAEC0ABC40BhAB0AzBjGF0ABJAnAEEjFfRBFdGffRBFdHff0DEByB"####,
    ),
    (
        "original-fixtures_direct_eval_scope",
        r####"(function(){var local=37;return eval("local + 5");})();
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ABJAnAENyBnAMAbyBn0ACJAnASzFjMjPjDjBjMBAndhFftZAnAEjzEjFj
WjBjMCfRBFeJjMjPjDjBjMhAhLhAhVffABB40BiAABAzADCAnf0DDByB"####,
    ),
    (
        "original-fixtures_do_while",
        r####"var n=0; do {n++;} while(n<3); n;
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ADJAnASzBjOByBndAftIAbAn0ABJAnATByBBtACzBhcCVBfyBnndDJAnA
VBfyBABB40BiAABAzADByB"####,
    ),
    (
        "original-fixtures_empty",
        r####"
"####,
        r####"@JSXBIN@ES@2.0@MyBn0DzABByB"####,
    ),
    (
        "original-fixtures_escaped_string",
        r####""line\nquote\"slash\\";
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ABJAnAFeRjMjJjOjFKjRjVjPjUjFhCjTjMjBjTjIic0DzABByB"####,
    ),
    (
        "original-fixtures_for_in",
        r####"var o={b:2,a:1}, k=[]; for(var p in o)k.push(p); k.sort().join("");
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ADbAn0ACJAnASzBjPByBWzGiPjCjKjFjDjUCCzBjCDFdCzBjBEFdBnftJ
AnASzBjLFyBAnnftLAJAnAEXzEjQjVjTjIGfVFfyBRBVzBjQHfyBffAVHfyBVBfyByBzAIfJAnAEXzE
jKjPjJjOJfEXzEjTjPjSjUKfVFfyBnfRBFeAffADB40BiAF4B0AiAH4C0AiAADAIByB"####,
    ),
    (
        "original-fixtures_for_while",
        r####"var s=0; for(var i=0;i<5;i++) s+=i; while(s<12) s++; s;
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0AEJAnASzBjTByBndAftaAJAnASByBCzBhLCnVzBjJDfyBnnntfAVDfyBA
FdFByBzBhcElAJAnATByBBtACEVBfyBnndMJAnAVBfyBACB40BiAD4B0AiAACAzAFByB"####,
    ),
    (
        "original-fixtures_function_constructor",
        r####"new Function("x", "return x * 4;")(3);
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ABJAnAEEjzIiGjVjOjDjUjJjPjOBfRCFeBjYFeNjSjFjUjVjSjOhAjYhA
hKhAhUhbftRBFdDff0DzACByB"####,
    ),
    (
        "original-fixtures_hoisting",
        r####"f(8); function f(x){return x+4;}
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBnABMAbyBn0ABZAnACzBhLBVzBjYCfAnndEABC40BhAB0AzBjGD0ABJAnAEj
DfRBFdIff0DzAEByB"####,
    ),
    (
        "original-fixtures_host_bridge_talk",
        r####"typeof BridgeTalk;
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ABJAnAizGjUjZjQjFjPjGBjzKiCjSjJjEjHjFiUjBjMjLCf0DzADByB"####,
    ),
    (
        "original-fixtures_host_explicit_property",
        r####"var c=app.project.items.addComp("Probe",64,64,1,1,24);var l=c.layers.addNull();l.property("ADBE Effect Parade").numProperties;
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ADJAnASzBjDByBEXzHjBjEjEiDjPjNjQCfXzFjJjUjFjNjTDfXzHjQjSj
PjKjFjDjUEfjzDjBjQjQFfRGFeFiQjSjPjCjFFdiAFdiAFdBFdBFdYffnftJAnASzBjMGyBEXzHjBjE
jEiOjVjMjMHfXzGjMjBjZjFjSjTIfVBfyBnfnftJAnAXzNjOjVjNiQjSjPjQjFjSjUjJjFjTJfEXzIj
QjSjPjQjFjSjUjZKfVGfyBRBFeSiBiEiCiFhAiFjGjGjFjDjUhAiQjBjSjBjEjFffACB40BiAG4B0Ai
AACAzALByB"####,
    ),
    (
        "original-fixtures_host_property_shortcut",
        r####"var c=app.project.items.addComp("Probe",64,64,1,1,24);var l=c.layers.addNull();l("ADBE Effect Parade").numProperties;
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ADJAnASzBjDByBEXzHjBjEjEiDjPjNjQCfXzFjJjUjFjNjTDfXzHjQjSj
PjKjFjDjUEfjzDjBjQjQFfRGFeFiQjSjPjCjFFdiAFdiAFdBFdBFdYffnftJAnASzBjMGyBEXzHjBjE
jEiOjVjMjMHfXzGjMjBjZjFjSjTIfVBfyBnfnftJAnAXzNjOjVjNiQjSjPjQjFjSjUjJjFjTJfEVGfy
BRBFeSiBiEiCiFhAiFjGjGjFjDjUhAiQjBjSjBjEjFffACB40BiAG4B0AiAACAzAKByB"####,
    ),
    (
        "original-fixtures_labelled_continue",
        r####"var n=0; outer: for(var i=0;i<3;i++){for(var j=0;j<3;j++){if(j===1)continue outer;n++;}} n;
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ADJAnASzBjOByBndAftaAbAn0ABaAbAn0ACOADAnAzFjPjVjUjFjSCfAC
zDhdhdhdDVzBjKEfyBnndBnJAnATByBBtAVEfyBAFdDByBzBhcFBCVzBjJGfyBAFdDByBFJAnAVBfyB
ADE4C0AiAB40BiAG4B0AiAADAzAHByB"####,
    ),
    (
        "original-fixtures_legacy_for_each",
        r####"var sum=0;for each(var value in [2,4,6])sum+=value;sum;
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ADJAnASzDjTjVjNByBndAftLAJAnASByBCzBhLCnVzFjWjBjMjVjFDfyB
nnntfAVDfyBARDFdCFdEFdGfyBzAEtJAnAVBfyBACB40BiAD4B0AiAACAEByB"####,
    ),
    (
        "original-fixtures_legacy_namespace",
        r####"var ns=new Namespace("urn:rig");String(ns);
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ACJAnASzCjOjTByBEjzJiOjBjNjFjTjQjBjDjFCfRBFeHjVjSjOhajSjJ
jHftnftJAnAEjzGiTjUjSjJjOjHDfRBVBfyBffABB40BiAABAzAEByB"####,
    ),
    (
        "original-fixtures_legacy_xml_constructor",
        r####"var x=new XML("<rig><bone/></rig>");x.children().length();
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ACJAnASzBjYByBEjzDiYiNiMCfRBFeShcjSjJjHhehcjCjPjOjFhPhehc
hPjSjJjHheftnftJAnAEXzGjMjFjOjHjUjIDfEXzIjDjIjJjMjEjSjFjOEfVBfyBnfnfABB40BiAABA
zAFByB"####,
    ),
    (
        "original-fixtures_legacy_xml_literal",
        r####"var x=<rig><bone id="1"/></rig>; String(x.bone.@id);
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ACJAnASzBjYByBoBFeZhcjSjJjHhehcjCjPjOjFhAjJjEhdhChRhChPhe
hchPjSjJjHheAnftJAnAEjzGiTjUjSjJjOjHCfRBXzDiAjJjEDfXzEjCjPjOjFEfVBfyBffABB40BiA
ABAzAFByB"####,
    ),
    (
        "original-fixtures_negative_zero",
        r####"1 / -0 === -Infinity;
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ABJAnACzDhdhdhdBnhzBhNCjzIiJjOjGjJjOjJjUjZDfd80EnQnfn0DzA
EByB"####,
    ),
    (
        "original-fixtures_numbers",
        r####"[1.25, 1e-4, 0xFE, -31].join("|");
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ABJAnAEXzEjKjPjJjOBfAREFd80EnUhfFd8hNiDgcnLnChWgahfFdneFd
ygffRBFeBjcff0DzACByB"####,
    ),
    (
        "original-fixtures_object_members",
        r####"var a={label:"rig", n:4}; a.n+=3; a["label"]+a.n;
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ADJAnASzBjBByBWzGiPjCjKjFjDjUCCzFjMjBjCjFjMDFeDjSjJjHzBjO
EFdEnftJAnABXEfVBfyBCzBhLFnnndDntJAnACFXDfVBfyBXEfVBfyBnnABB40BiAABAzAGByB"####,
    ),
    (
        "original-fixtures_regexp",
        r####""rig01 rig22".match(/rig[0-9]+/g).join("|");
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ABJAnAEXzEjKjPjJjOBfEXzFjNjBjUjDjICfFeLjSjJjHhQhRhAjSjJjH
hShSRBYJjSjJjHibhQhNhZidhLBjHffRBFeBjcff0DzADByB"####,
    ),
    (
        "original-fixtures_scriptui_palette",
        r####"var w=new Window("palette","Probe");w.add("button",undefined,"Build");w.show();w.children.length;
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0AEJAnASzBjXByBEjzGiXjJjOjEjPjXCfRCFeHjQjBjMjFjUjUjFFeFiQj
SjPjCjFftnftJAnAEXzDjBjEjEDfVBfyBRDFeGjCjVjUjUjPjOjzJjVjOjEjFjGjJjOjFjEEfFeFiCj
VjJjMjEffJAnAEXzEjTjIjPjXFfVBfyBnfJAnAXzGjMjFjOjHjUjIGfXzIjDjIjJjMjEjSjFjOHfVBf
yBABB40BiAABAzAIByB"####,
    ),
    (
        "original-fixtures_switch_fallthrough",
        r####"var s=""; switch(2){case 1:s+="a";break;case 2:s+="b";case 3:s+="c";break;default:s="x";} s;
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ADJAnASzBjTByBneAftcAnAFdCERBFdBfRBFdCfRBFdDfRBnfEbAn0ACJ
AnASByBCzBhLCnnneBjBntfDAnAzADtbAn0ABJAnASByBCCnnneBjCntfbAn0ACJAnASByBCCnnneBj
DntfDAnADtbAn0ABJAnASByBneBjYffJAnAVBfyBABB40BiAABADByB"####,
    ),
    (
        "original-fixtures_this_call",
        r####"var o={n:9}; function f(x){return this.n+x;}f.call(o,4);
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBnABMAbyBn0ABZAnACzBhLBXzBjOCfezEjUjIjJjTDfVzBjYEfAnnABE40Bh
AB0AzBjGF0ACJAnASzBjPGyBWzGiPjCjKjFjDjUHBCFdJnftJAnAEXzEjDjBjMjMIfjFfRCVGfyBFdE
ffABG40BiAABAzAJByB"####,
    ),
    (
        "original-fixtures_try_finally",
        r####"var s=""; try{throw new Error("x");}catch(e){s=e.message;}finally{s+="!";}s;
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ADJAnASzBjTByBneAftgAbyBn0ABfAnAEjzFiFjSjSjPjSCfRBFeBjYft
ABbyBn0ABJAnASByBCzBhLDnnneBhBntfzBjFEnbyBn0ABJAnASByBXzHjNjFjTjTjBjHjFFfjEfnff
JAnAVBfyBABB40BiAABAzAGByB"####,
    ),
    (
        "original-fixtures_unicode",
        r####""Rim \u00e4 \u65e5 \ud83c\udfa5";
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ABJAnAFeKiSjJjNhAnEhA2nFjFhA2hcmY2lFmf0DzABByB"####,
    ),
    (
        "original-fixtures_with_scope",
        r####"var x=2,o={x:8}; with(o){x+=1;} x+":"+o.x;
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ADbAn0ACJAnASzBjYByBndCftJAnASzBjPCyBWzGiPjCjKjFjDjUDBBFd
InftmAbAn0ABJAnASByBCzBhLEnnndBntfAVCfyBJAnACECEVBfyBnneBhaXBfVCfyBnnACB40BiAC4
B0AiAACAzAFByB"####,
    ),
    (
        "source_array",
        r####"[1,2,,4].join("|");
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ABJAnAEXzEjKjPjJjOBfAREFdBFdCnFdEfRBFeBjcff0DzACByB"####,
    ),
    (
        "source_assignment",
        r####"var a=3;a=5;a;
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ADJAnASzBjBByBndDftJAnASByBndFffJAnAVBfyBABB40BiAABAzACBy
B"####,
    ),
    (
        "source_comma",
        r####"var a=1;(a++,a+=2,a);
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ACJAnASzBjBByBndBftbAn0ADJAnATByBBtJAnASByBCzBhLCnnndCntf
JAnAVBfyBABB40BiAABAzADByB"####,
    ),
    (
        "source_compound",
        r####"var a=5;a+=2;a*=3;a-=1;a/=2;a%=7;a;
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0AHJAnASzBjBByBndFftJAnASByBCzBhLCnnndCntfJAnASByBCzBhKDnn
ndDntfJAnASByBCzBhNEnnndBntfJAnASByBCzBhPFnnndCntfJAnASByBCzBhFGnnndHntfJAnAVBf
yBABB40BiAABAzAHByB"####,
    ),
    (
        "source_conditional",
        r####"var x=5;x>3?"yes":"no";
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ACJAnASzBjYByBndFftJAnAdCzBheCVBfyBnndDFeDjZjFjTFeCjOjPAB
B40BiAABAzADByB"####,
    ),
    (
        "source_debugger",
        r####"debugger;7;
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ACHAnAJAnAFdH0DzABByB"####,
    ),
    (
        "source_declaration",
        r####"var a;
"####,
        r####"@JSXBIN@ES@2.0@MyBnABzBjBB40BiAABAzACByB"####,
    ),
    (
        "source_delete",
        r####"var o={a:1,b:2};delete o.a;Object.prototype.hasOwnProperty.call(o,"a");
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ADJAnASzBjPByBWzGiPjCjKjFjDjUCCzBjBDFdBzBjCEFdCnftJAnAizG
jEjFjMjFjUjFFXDfVBfyBJAnAEXzEjDjBjMjMGfXzOjIjBjTiPjXjOiQjSjPjQjFjSjUjZHfXzJjQjS
jPjUjPjUjZjQjFIfjCfRCVBfyBFeBjBffABB40BiAABAzAJByB"####,
    ),
    (
        "source_do",
        r####"var a=0;do a++;while(a<4);a;
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ADJAnASzBjBByBndAftIAJAnATByBBtACzBhcCVBfyBnndEJAnAVBfyBA
BB40BiAABAzADByB"####,
    ),
    (
        "source_e4x_descendant",
        r####"var x=<root><group><a/></group></root>;x..a.length();
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ACJAnASzBjYByBoBFehAhcjSjPjPjUhehcjHjSjPjVjQhehcjBhPhehch
PjHjSjPjVjQhehchPjSjPjPjUheAnftJAnAEXzGjMjFjOjHjUjICfqzBjBDfVBfyBnnnfABB40BiAAB
AzAEByB"####,
    ),
    (
        "source_e4x_embed",
        r####"var s="name";var n=3;(<a id={n}>{s}</a>).toXMLString();
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ADJAnASzBjTByBneEjOjBjNjFftJAnASzBjOCyBndDftJAnAEXzLjUjPi
YiNiMiTjUjSjJjOjHDfoFFeGhcjBhAjJjEhdAbyBn0ABJAnAVCfyBCFeBheAbyBn0ABJAnAVBfyBDFe
EhchPjBheAnfACB40BiAC4B0AiAACAzAEByB"####,
    ),
    (
        "source_e4x_filter",
        r####"var x=<root><a id="1"/><a id="2"/></root>;x.a.(@id=="2").length();
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ACJAnASzBjYByBoBFehDhcjSjPjPjUhehcjBhAjJjEhdhChRhChPhehcj
BhAjJjEhdhChShChPhehchPjSjPjPjUheAnftJAnAEXzGjMjFjOjHjUjICfrzADfXzBjBEfVBfyBCzC
hdhdFjzDiAjJjEGfnneBhSnfABB40BiAABADByB"####,
    ),
    (
        "source_e4x_list",
        r####"var x=<><a/><b/></>;x.length();
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ACJAnASzBjYByBoBFeNhchehcjBhPhehcjChPhehchPheAnftJAnAEXzG
jMjFjOjHjUjICfVBfyBnfABB40BiAABAzADByB"####,
    ),
    (
        "source_e4x_literal",
        r####"var x=<rig><bone id="1"/></rig>;x.bone.@id.toString();
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ACJAnASzBjYByBoBFeZhcjSjJjHhehcjCjPjOjFhAjJjEhdhChRhChPhe
hchPjSjJjHheAnftJAnAEXzIjUjPiTjUjSjJjOjHCfXzDiAjJjEDfXzEjCjPjOjFEfVBfyBnfABB40B
iAABAzAFByB"####,
    ),
    (
        "source_e4x_namespace",
        r####"var ns=new Namespace("urn:rig");default xml namespace=ns;(<a/>).namespace().uri;
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ADJAnASzCjOjTByBEjzJiOjBjNjFjTjQjBjDjFCfRBFeHjVjSjOhajSjJ
jHftnftJAnAkEjzWjTjFjUiEjFjGjBjVjMjUiYiNiMiOjBjNjFjTjQjBjDjFDfRBVBfyBffJAnAXzDj
VjSjJEfEXzJjOjBjNjFjTjQjBjDjFFfoBFeEhcjBhPheAnfABB40BiAABAzAGByB"####,
    ),
    (
        "source_eval_direct",
        r####"(function(){var local=37;return eval("local+5");})();
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ABJAnAENyBnAMAbyBn0ACJAnASzFjMjPjDjBjMBAndhFftZAnAEjzEjFj
WjBjMCfRBFeHjMjPjDjBjMhLhVffABB40BiAABAzADCAnf0DDByB"####,
    ),
    (
        "source_fn_arguments",
        r####"function f(){return arguments.length+arguments[0];}f(2,3);
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBnABMAbyBn0ABZAnACzBhLBXzGjMjFjOjHjUjICfVzJjBjSjHjVjNjFjOjUj
TDf4kBkWkYAXzBhQEfVDf4kBkWkYAnn0DzBjGF0ABJAnAEjFfRCFdCFdDff0DzAGByB"####,
    ),
    (
        "source_fn_closure",
        r####"function outer(x){return function(y){return x+y;};}outer(3)(4);
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBnABMAbyBn0ABZAnANyBnAMAbyBn0ABZAnACzBhLBjzBjYCfVzBjZDfAnnAB
D40BhAB0AzAEC0ABC40BhAB0AzFjPjVjUjFjSF0ABJAnAEEjFfRBFdDffRBFdEff0DEByB"####,
    ),
    (
        "source_fn_empty",
        r####"function f(){}f();
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBnABMAn0DzBjGB0ABJAnAEjBfnf0DzACByB"####,
    ),
    (
        "source_fn_named_expr",
        r####"var f=function named(n){return n<2?1:n*named(n-1);};f(5);
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ACJAnASzBjGByBNyBnAMAbyBn0ABZAnAdCzBhcCVzBjODfAnndCFdBCzB
hKEVDfAEjzFjOjBjNjFjEFfRBCzBhNGVDfAnndBffnnABD40BhAB0AFCAnftJAnAEVBfyBRBFdFffAB
B40BiAABAzAHByB"####,
    ),
    (
        "source_fn_return",
        r####"function f(a,b){var c=a+b;return c;}f(2,5);
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBnABMAbyBn0ACJAnASzBjDBACzBhLCVzBjBDfBVzBjCEfCnnnftZAnAVBf0A
DD40BhAB40BiAE4B0AhACBAzBjGF0ABJAnAEjFfRCFdCFdFff0DzAGByB"####,
    ),
    (
        "source_for",
        r####"var a=0;for(var i=0;i<4;i++){a+=i;}a;
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ADJAnASzBjBByBndAftaAbAn0ABJAnASByBCzBhLCnVzBjJDfyBnnntfA
VDfyBAFdEByBzBhcEJAnAVBfyBACB40BiAD4B0AiAACAzAFByB"####,
    ),
    (
        "source_for_each",
        r####"var s=0;for each(var v in [2,4,6])s+=v;s;
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ADJAnASzBjTByBndAftLAJAnASByBCzBhLCnVzBjWDfyBnnntfAVDfyBA
RDFdCFdEFdGfyBzAEtJAnAVBfyBACD4B0AiAB40BiAACAEByB"####,
    ),
    (
        "source_for_empty",
        r####"var a=0;for(;;){a++;if(a==3)break;}a;
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ADJAnASzBjBByBndAftlAbAn0ACJAnATByBBtOADAnAzACtACzChdhdDV
BfyBnndDnAFctJAnAVBfyBABB40BiAABACByB"####,
    ),
    (
        "source_for_in",
        r####"var s="",x={a:1,b:2};for(var k in x)s+=k;s;
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ADbAn0ACJAnASzBjTByBneAftJAnASzBjYCyBWzGiPjCjKjFjDjUDCzBj
BEFdBzBjCFFdCnftLAJAnASByBCzBhLGnVzBjLHfyBnnntfAVHfyBVCfyByBzAIfJAnAVBfyBADC4B0
AiAH4C0AiAB40BiAADAIByB"####,
    ),
    (
        "source_identifier",
        r####"demo;
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ABJAnAjzEjEjFjNjPBf0DzACByB"####,
    ),
    (
        "source_if_else",
        r####"var a=4;if(a>2){a+=8;}else{a-=3;}a;
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ADJAnASzBjBByBndEftOAbAn0ABJAnASByBCzBhLCnnndIntfACzBheDV
BfyBnndCbAn0ABJAnASByBCzBhNEnnndDntfJAnAVBfyBABB40BiAABAzAFByB"####,
    ),
    (
        "source_if_empty",
        r####"var a=1;if(a);a;
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ADJAnASzBjBByBndBftOAnAVBfyBnJAnAVBfyBABB40BiAABAzACByB"####,
    ),
    (
        "source_increment",
        r####"var a=2;var b=a++;var c=++a;a+":"+b+":"+c;
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0AEJAnASzBjBByBndCftJAnASzBjCCyBTByBBtnftJAnASzBjDDyBTByBB
fnftJAnACzBhLECECECEVBfyBnneBhaVCfyBnnnneBhaVDfyBnnADB40BiAD4C0AiAC4B0AiAADAzAF
ByB"####,
    ),
    (
        "source_label_break",
        r####"var n=0;outer:for(var i=0;i<3;i++){for(var j=0;j<3;j++){n++;if(j==1)break outer;}}n;
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ADJAnASzBjOByBndAftaAbAn0ABaAbAn0ACJAnATByBBtOADAnAzFjPjV
jUjFjSCtACzChdhdDVzBjKEfyBnndBnAVEfyBAFdDByBzBhcFBCVzBjJGfyBAFdDByBFJAnAVBfyBAD
E4C0AiAG4B0AiAB40BiAADAzAHByB"####,
    ),
    (
        "source_literal_false",
        r####"false;
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ABJAnAFcf0DzABByB"####,
    ),
    (
        "source_literal_float",
        r####"1.123456789;
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ABJAnAFd8hWkWjTmTlNnZnRhf0DzABByB"####,
    ),
    (
        "source_literal_int_255",
        r####"255;
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ABJAnAFdnf0DzABByB"####,
    ),
    (
        "source_literal_int_256",
        r####"256;
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ABJAnAFd2AB0DzABByB"####,
    ),
    (
        "source_literal_int_65536",
        r####"65536;
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ABJAnAFd40AB0EzABByB"####,
    ),
    (
        "source_literal_int_negative",
        r####"-2147483648;
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ABJAnAFd40BkA0DzABByB"####,
    ),
    (
        "source_literal_null",
        r####"null;
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ABJAnAFb0DzABByB"####,
    ),
    (
        "source_literal_true",
        r####"true;
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ABJAnAFct0DzABByB"####,
    ),
    (
        "source_literal_undefined",
        r####"undefined;
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ABJAnAjzJjVjOjEjFjGjJjOjFjEBf0DzACByB"####,
    ),
    (
        "source_logic",
        r####"var a=0,b=5;(a||b)+(a&&b);
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ACbAn0ACJAnASzBjBByBndAftJAnASzBjCCyBndFftJAnACzBhLDUzCjc
jcEVBfyBVCfyBnnUzChGhGFVBfyBVCfyBnnnnACB40BiAC4B0AiAACAzAGByB"####,
    ),
    (
        "source_new",
        r####"function Box(x){this.x=x;}var b=new Box(7);b.x;
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBnABMAbyBn0ABJAnABXzBjYBfezEjUjIjJjTCfVBfAnfABB40BhAB0AzDiCj
PjYD0ACJAnASzBjCEyBEjDfRBFdHftnftJAnAXBfVEfyBABE40BiAABAzAFByB"####,
    ),
    (
        "source_object",
        r####"({a:1,"b-c":2})["b-c"];
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ABJAnAXzDjChNjDBfWzGiPjCjKjFjDjUCCzBjBDFdBBFdC0DzAEByB"####,
    ),
    (
        "source_operators",
        r####"var a=9,b=3;[a+b,a-b,a*b,a/b,a%b,a<<b,a>>b,a>>>b,a&b,a|b,a^b,a<b,a>b,a<=b,a>=b,a==b,a===b,a!=b,a!==b].join("|");
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ACbAn0ACJAnASzBjBByBndJftJAnASzBjCCyBndDftJAnAEXzEjKjPjJj
ODfARTCzBhLEVBfyBVCfyBnnCzBhNFVBfyBVCfyBnnCzBhKGVBfyBVCfyBnnCzBhPHVBfyBVCfyBnnC
zBhFIVBfyBVCfyBnnCzChchcJVBfyBVCfyBnnCzCheheKVBfyBVCfyBnnCzDheheheLVBfyBVCfyBnn
CzBhGMVBfyBVCfyBnnCzBjcNVBfyBVCfyBnnCzBieOVBfyBVCfyBnnCzBhcPVBfyBVCfyBnnCzBheQV
BfyBVCfyBnnCzChchdRVBfyBVCfyBnnCzChehdSVBfyBVCfyBnnCzChdhdTVBfyBVCfyBnnCzDhdhdh
dUVBfyBVCfyBnnCzChBhdVVBfyBVCfyBnnCzDhBhdhdWVBfyBVCfyBnnfRBFeBjcffACB40BiAC4B0A
iAACAzAXByB"####,
    ),
    (
        "source_regexp",
        r####"/ab+c/gi.test("ABBC");
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ABJAnAEXzEjUjFjTjUBfYEjBjChLjDCjHjJRBFeEiBiCiCiDff0DzACBy
B"####,
    ),
    (
        "source_switch",
        r####"var a=2,b=0;switch(a){case 1:b=1;break;case 2:b=2;default:b+=3;}b;
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ADbAn0ACJAnASzBjBByBndCftJAnASzBjCCyBndAftcAnAVBfyBDRBFdB
fRBFdCfRBnfDbAn0ACJAnASCyBndBffDAnAzADtbAn0ABJAnASCyBndCffbAn0ABJAnASCyBCzBhLEn
nndDntfJAnAVCfyBACB40BiAC4B0AiAACADByB"####,
    ),
    (
        "source_try",
        r####"var r="";try{throw "x";}catch(e){r=e;}finally{r+="!";}r;
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ADJAnASzBjSByBneAftgAbyBn0ABfAnAFeBjYABbyBn0ABJAnASByBCzB
hLCnnneBhBntfzBjFDnbyBn0ABJAnASByBjDfnffJAnAVBfyBABB40BiAABAzAEByB"####,
    ),
    (
        "source_try_guarded",
        r####"var r="";try{throw 3;}catch(e if e===3){r="three";}catch(e){r="other";}r;
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ADJAnASzBjSByBneAftgAbyBn0ABfAnAFdDACnzBjFCCzDhdhdhdDjCfn
ndDbyBn0ABJAnASByBneFjUjIjSjFjFffCnbyBn0ABJAnASByBneFjPjUjIjFjSffJAnAVBfyBABB40
BiAABAzAEByB"####,
    ),
    (
        "source_ui_callback",
        r####"var w=new Window("palette","Compiled Panel");var b=w.add("button",undefined,"Run");b.onClick=function(){app.project.items.addComp("From compiled panel",64,64,1,1,24);};w.show();
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0AEJAnASzBjXByBEjzGiXjJjOjEjPjXCfRCFeHjQjBjMjFjUjUjFFeOiDj
PjNjQjJjMjFjEhAiQjBjOjFjMftnftJAnASzBjCDyBEXzDjBjEjEEfVBfyBRDFeGjCjVjUjUjPjOjzJ
jVjOjEjFjGjJjOjFjEFfFeDiSjVjOffnftJAnABXzHjPjOiDjMjJjDjLGfVDfyBNyBnAMAbyBn0ABJA
nAEXzHjBjEjEiDjPjNjQHfXzFjJjUjFjNjTIfXzHjQjSjPjKjFjDjUJfjzDjBjQjQKfRGFeTiGjSjPj
NhAjDjPjNjQjJjMjFjEhAjQjBjOjFjMFdiAFdiAFdBFdBFdYff0DzALCAnfJAnAEXzEjTjIjPjXMfVB
fyBnfACD4B0AiAB40BiAACALByB"####,
    ),
    (
        "source_unary",
        r####"var a=2;[!a,~a,+a,-a,typeof a,void a].join("|");
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ACJAnASzBjBByBndCftJAnAEXzEjKjPjJjOCfARGhzBhBDVBfyBhzBjeE
VBfyBhzBhLFVBfyBhzBhNGVBfyBizGjUjZjQjFjPjGHVBfyBhzEjWjPjJjEIVBfyBfRBFeBjcffABB4
0BiAABAzAJByB"####,
    ),
    (
        "source_var_multi",
        r####"var a=1,b=2,c=3;a+b+c;
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ACbAn0ADJAnASzBjBByBndBftJAnASzBjCCyBndCftJAnASzBjDDyBndD
ftJAnACzBhLECEVBfyBVCfyBnnVDfyBnnADB40BiAD4C0AiAC4B0AiAADAzAFByB"####,
    ),
    (
        "source_while",
        r####"var a=0;while(a<4)a++;a;
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ADJAnASzBjBByBndAftlAJAnATByBBtACzBhcCVBfyBnndEJAnAVBfyBA
BB40BiAABAzADByB"####,
    ),
    (
        "source_with",
        r####"var x={value:4};with(x){value+=3;}x.value;
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ADJAnASzBjYByBWzGiPjCjKjFjDjUCBzFjWjBjMjVjFDFdEnftmAbAn0A
BJAnABjDfCzBhLEnnndDntAVBfyBJAnAXDfVBfyBABB40BiAABAzAFByB"####,
    ),
    (
        "extended_array_holes_tail",
        r####"var a=[,,]; a.length+":"+(0 in a)+":"+(1 in a);
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ACJAnASzBjBByBARCnnfnftJAnACzBhLCCCCCCCXzGjMjFjOjHjUjIDfV
BfyBnneBhaCzCjJjOEnVBfyBdAnnnnneBhaCEnVBfyBdBnnnABB40BiAABAzAFByB"####,
    ),
    (
        "extended_array_undefined",
        r####"var a=[undefined,null,,]; a.length+":"+(0 in a)+":"+(2 in a);
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ACJAnASzBjBByBARDjzJjVjOjEjFjGjJjOjFjECfFbnfnftJAnACzBhLD
CDCDCDXzGjMjFjOjHjUjIEfVBfyBnneBhaCzCjJjOFnVBfyBdAnnnnneBhaCFnVBfyBdCnnnABB40Bi
AABAzAGByB"####,
    ),
    (
        "extended_comma_nested",
        r####"var a=0; var b=(a=1,a+2); b+":"+a;
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ADJAnASzBjBByBndAftJAnASzBjCCyBRCSByBndBffCzBhLDVBfyBnndC
tnftJAnACDCDVCfyBnneBhaVBfyBnnACB40BiAC4B0AiAACAzAEByB"####,
    ),
    (
        "extended_compound_member",
        r####"var a={x:2}; var i=0; a[(i++,"x")]+=3; a.x+":"+i;
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0AEJAnASzBjBByBWzGiPjCjKjFjDjUCBzBjYDFdCnftJAnASzBjJEyBndA
ftJAnABQzAFfVBfyBRCTEyBBtFeBjYtCzBhLGnnndDntJAnACGCGXDfVBfyBnneBhaVEfyBnnACB40B
iAE4B0AiAACAFByB"####,
    ),
    (
        "extended_const_decl",
        r####"const a=3; a;
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ACJAnAGzBjBByBndDftJAnAVBfyBABB40BjA0ABzACByB"####,
    ),
    (
        "extended_e4x_add",
        r####"var a=<a/>,b=<b/>;var c=a+b;c.length();
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ADbAn0ACJAnASzBjBByBoBFeEhcjBhPheAnftJAnASzBjCCyBoBFeEhcj
ChPheAnftJAnASzBjDDyBCzBhLEVBfyBVCfyBnnnftJAnAEXzGjMjFjOjHjUjIFfVDfyBnfADB40BiA
C4B0AiAD4C0AiAADAzAGByB"####,
    ),
    (
        "extended_e4x_any_namespace",
        r####"var x=<a><b>yes</b></a>;x.*::b.toString();
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ACJAnASzBjYByBoBFeRhcjBhehcjChejZjFjThchPjChehchPjBheAnft
JAnAEXzIjUjPiTjUjSjJjOjHCfpzBhKDfVBfyBnnzBjCEnfABB40BiAABAzAFByB"####,
    ),
    (
        "extended_e4x_assign",
        r####"var x=<a><b>old</b></a>;x.b="new";x.@id="2";x.toXMLString();
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0AEJAnASzBjYByBoBFeRhcjBhehcjChejPjMjEhchPjChehchPjBheAnft
JAnABXzBjCCfVBfyBneDjOjFjXfJAnABXzDiAjJjEDfVBfyBneBhSfJAnAEXzLjUjPiYiNiMiTjUjSj
JjOjHEfVBfyBnfABB40BiAABAzAFByB"####,
    ),
    (
        "extended_e4x_attr",
        r####"var x=<a id="1"/>; x.@id.toString();
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ACJAnASzBjYByBoBFeLhcjBhAjJjEhdhChRhChPheAnftJAnAEXzIjUjP
iTjUjSjJjOjHCfXzDiAjJjEDfVBfyBnfABB40BiAABAzAEByB"####,
    ),
    (
        "extended_e4x_delete",
        r####"var x=<a><b/><c/></a>;delete x.b;x.toXMLString();
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ADJAnASzBjYByBoBFePhcjBhehcjChPhehcjDhPhehchPjBheAnftJAnA
izGjEjFjMjFjUjFCXzBjCDfVBfyBJAnAEXzLjUjPiYiNiMiTjUjSjJjOjHEfVBfyBnfABB40BiAABAz
AFByB"####,
    ),
    (
        "extended_e4x_desc_attr",
        r####"var x=<a><b id="1"/><b id="2"/></a>;x..@id.toXMLString();
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ACJAnASzBjYByBoBFegdhcjBhehcjChAjJjEhdhChRhChPhehcjChAjJj
EhdhChShChPhehchPjBheAnftJAnAEXzLjUjPiYiNiMiTjUjSjJjOjHCfqzDiAjJjEDfVBfyBnnnfAB
B40BiAABAzAEByB"####,
    ),
    (
        "extended_e4x_dynamic_attr",
        r####"var x=<a id="1"/>;var k="id";x.@[k].toString();
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ADJAnASzBjYByBoBFeLhcjBhAjJjEhdhChRhChPheAnftJAnASzBjLCyB
neCjJjEftJAnAEXzIjUjPiTjUjSjJjOjHDfQzAEtVBfyBVCfyBnfACB40BiAC4B0AiAACAEByB"####,
    ),
    (
        "extended_e4x_dynamic_child",
        r####"var x=<a><b>yes</b></a>;var k="b";x[k].toString();
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ADJAnASzBjYByBoBFeRhcjBhehcjChejZjFjThchPjChehchPjBheAnft
JAnASzBjLCyBneBjCftJAnAEXzIjUjPiTjUjSjJjOjHDfQzAEfVBfyBVCfyBnfACB40BiAC4B0AiAAC
AEByB"####,
    ),
    (
        "extended_e4x_embed_escape",
        r####"var v="a&<\"";var x=<a b={v}>{v}</a>;x.toXMLString();
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ADJAnASzBjWByBneEjBhGhchCftJAnASzBjYCyBoFFeFhcjBhAjChdAby
Bn0ABJAnAVBfyBCFeBheAbyBn0ABJAnAVBfyBDFeEhchPjBheAnftJAnAEXzLjUjPiYiNiMiTjUjSjJ
jOjHDfVCfyBnfACB40BiAC4B0AiAACAzAEByB"####,
    ),
    (
        "extended_e4x_filter",
        r####"var x=<root><a id="1"/><a id="2"/></root>; x.a.(@id=="2").length();
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ACJAnASzBjYByBoBFehDhcjSjPjPjUhehcjBhAjJjEhdhChRhChPhehcj
BhAjJjEhdhChShChPhehchPjSjPjPjUheAnftJAnAEXzGjMjFjOjHjUjICfrzADfXzBjBEfVBfyBCzC
hdhdFjzDiAjJjEGfnneBhSnfABB40BiAABADByB"####,
    ),
    (
        "extended_e4x_name_embed",
        r####"var v="b";var x=<{v}>yes</{v}>;x.toXMLString();
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ADJAnASzBjWByBneBjCftJAnASzBjYCyBoFFeBhcAbyBn0ABJAnAVBfyB
BFeGhejZjFjThchPAbyBn0ABJAnAVBfyBBFeBheAnftJAnAEXzLjUjPiYiNiMiTjUjSjJjOjHDfVCfy
BnfACB40BiAC4B0AiAACAzAEByB"####,
    ),
    (
        "extended_e4x_qname",
        r####"var ns=new Namespace("n","urn:x");var x=<a xmlns:n="urn:x"><n:b>yes</n:b></a>;x.ns::b.toString();
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ADJAnASzCjOjTByBEjzJiOjBjNjFjTjQjBjDjFCfRCFeBjOFeFjVjSjOh
ajYftnftJAnASzBjYDyBoBFehFhcjBhAjYjNjMjOjThajOhdhCjVjSjOhajYhChehcjOhajChejZjFj
ThchPjOhajChehchPjBheAnftJAnAEXzIjUjPiTjUjSjJjOjHEfpBfVDfyBnnzBjCFnfACB40BiAD4B
0AiAACAzAGByB"####,
    ),
    (
        "extended_e4x_qname_attr",
        r####"var ns=new Namespace("n","urn:x");var x=<a xmlns:n="urn:x" n:b="yes"/>;x.@ns::b.toString();
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ADJAnASzCjOjTByBEjzJiOjBjNjFjTjQjBjDjFCfRCFeBjOFeFjVjSjOh
ajYftnftJAnASzBjYDyBoBFegehcjBhAjYjNjMjOjThajOhdhCjVjSjOhajYhChAjOhajChdhCjZjFj
ThChPheAnftJAnAEXzIjUjPiTjUjSjJjOjHEfpBtVDfyBnnzBjCFnfACB40BiAD4B0AiAACAzAGByB"####,
    ),
    (
        "extended_e4x_typeof",
        r####"typeof <a/>;
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ABJAnAhzGjUjZjQjFjPjGBoBFeEhcjBhPhe0EzACByB"####,
    ),
    (
        "extended_eval_payload",
        r####"var local=local+2; var added=9; local+added;
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ADJAnASzFjMjPjDjBjMByBCzBhLCVBfyBnndCnftJAnASzFjBjEjEjFjE
DyBndJftJAnACCVBfyBVDfyBnnACB40BiAD4B0AiAACAzAEByB"####,
    ),
    (
        "extended_fn_params",
        r####"function f(z,a,q){return z+":"+a+":"+q;} f(1,2,3);
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBnABMAbyBn0ABZAnACzBhLBCBCBCBVzBjaCfAnneBhaVzBjBDfBnnnneBhaV
zBjREfCnnADD4B0AhAC40BhAE4C0AhAD0AzBjGF0ABJAnAEjFfRDFdBFdCFdDff0DzAGByB"####,
    ),
    (
        "extended_for_assign_existing",
        r####"var x="",o={a:1}; for(x in o){} x;
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ADbAn0ACJAnASzBjYByBneAftJAnASzBjPCyBWzGiPjCjKjFjDjUDBzBj
BEFdBnftLAnAVBfyBVCfyByBzAFfJAnAVBfyBACB40BiAC4B0AiAACAFByB"####,
    ),
    (
        "extended_for_each_member",
        r####"var o={},s=0; for each(o.x in {a:1,b:2})s+=o.x; s;
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ADbAn0ACJAnASzBjPByBWzGiPjCjKjFjDjUCAnftJAnASzBjTDyBndAft
LAJAnASDyBCzBhLEnXzBjYFfVBfyBnnntfAXFfVBfyBWCCzBjBGFdBzBjCHFdCyBzAItJAnAVDfyBAC
B40BiAD4B0AiAACAIByB"####,
    ),
    (
        "extended_for_member",
        r####"var o={},s=""; for(o.x in {a:1,b:2})s+=o.x; s;
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ADbAn0ACJAnASzBjPByBWzGiPjCjKjFjDjUCAnftJAnASzBjTDyBneAft
LAJAnASDyBCzBhLEnXzBjYFfVBfyBnnntfAXFfVBfyBWCCzBjBGFdBzBjCHFdCyBzAIfJAnAVDfyBAC
B40BiAD4B0AiAACAIByB"####,
    ),
    (
        "extended_for_multi",
        r####"var s=0; for(var i=0,j=3;i<j;i++,j--){s+=i+j;} s;
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ADJAnASzBjTByBndAftKAbAn0ABJAnASByBCzBhLCnCCVzBjJDfyBVzBj
KEfyBnnnnntfARCSDyBndAftSEyBndDfttCzBhcFVDfyBVEfyBnnRCTDyBBtTEyByBttJAnAVBfyBAD
D4B0AiAB40BiAE4C0AiAADAzAGByB"####,
    ),
    (
        "extended_guarded_chain",
        r####"var s="";try{throw 2;}catch(e if e===1){s="one";}catch(e if e===2){s="two";}catch(e){s="other";} s;
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ADJAnASzBjTByBneAftgAbyBn0ABfAnAFdCADnzBjFCCzDhdhdhdDjCfn
ndBbyBn0ABJAnASByBneDjPjOjFffCCDjCfnndCbyBn0ABJAnASByBneDjUjXjPffCnbyBn0ABJAnAS
ByBneFjPjUjIjFjSffJAnAVBfyBABB40BiAABAzAEByB"####,
    ),
    (
        "extended_guarded_finally",
        r####"var s="";try{try{throw 3;}catch(e if e===1){s="one";}finally{s+="f";}}catch(e){s+=e;}s;
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ADJAnASzBjTByBneAftgAbyBn0ABgAbyBn0ABfAnAFdDABbyBn0ABJAnA
SByBCzBhLCnnneBjGntfzBjFDCzDhdhdhdEjDfnndBbyBn0ABJAnASByBneDjPjOjFffABnDnbyBn0A
BJAnASByBCCnjDfnnntfJAnAVBfyBABB40BiAABAzAFByB"####,
    ),
    (
        "extended_negative_values",
        r####"[-2147483648,-2147483647,2147483647,2147483648,4294967295,-65536].join("|");
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ABJAnAEXzEjKjPjJjOBfARGFd40BkAFdy4nfnfnfjfFd4nfnfnfjfFd80
EnAiBFd80AnAnfnfnfnPiBFdy40ABAfRBFeBjcff0DzACByB"####,
    ),
    (
        "extended_negative_zero_literal",
        r####"var z=-0; 1/z;
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ACJAnASzBjaByBnd80FkAftJAnACzBhPCnVBfyBdBnABB40BiAABAzADB
yB"####,
    ),
    (
        "extended_panel_this",
        r####"this.add("button",undefined,"Compiled Button").onClick=function(){ app.project.items.addComp("Compiled Callback",320,240,1,2,24); };
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ABJAnABXzHjPjOiDjMjJjDjLBfEXzDjBjEjECfezEjUjIjJjTDfRDFeGj
CjVjUjUjPjOjzJjVjOjEjFjGjJjOjFjEEfFePiDjPjNjQjJjMjFjEhAiCjVjUjUjPjOffNyBnAMAbyB
n0ABJAnAEXzHjBjEjEiDjPjNjQFfXzFjJjUjFjNjTGfXzHjQjSjPjKjFjDjUHfjzDjBjQjQIfRGFeRi
DjPjNjQjJjMjFjEhAiDjBjMjMjCjBjDjLFd2iABFdnQFdBFdCFdYff0DzAJCAnf0DJByB"####,
    ),
    (
        "extended_string_keys",
        r####"var x={"a\"b":3,"__proto__":7}; x["a\"b"]+":"+x["__proto__"];
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ACJAnASzBjYByBWzGiPjCjKjFjDjUCCzDjBhCjCDFdDzJififjQjSjPjU
jPififEFdHnftJAnACzBhLFCFXDfVBfyBnneBhaXEfVBfyBnnABB40BiAABAzAGByB"####,
    ),
    (
        "extended_switch_middle_default",
        r####"var s=""; switch(4){case 1:s+="a";break;default:s+="d";case 2:s+="b";} s;
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ADJAnASzBjTByBneAftcAnAFdEDRBFdBfRBnfRBFdCfDbAn0ACJAnASBy
BCzBhLCnnneBjBntfDAnAzADtbAn0ABJAnASByBCCnnneBjEntfbAn0ABJAnASByBCCnnneBjCntfJA
nAVBfyBABB40BiAABADByB"####,
    ),
    (
        "regressions_block_function",
        r####"var f; if(true){function f(){return 7;} var f; } f();
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ACbAnABMAbyBn0ABZAnAFdH0DzBjGB0BJAnAEVBfyBnfABB40BiAABAzA
CByB"####,
    ),
    (
        "regressions_function_hoist",
        r####"var n=f();function f(){return 7;}n;
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBnABMAbyBn0ABZAnAFdH0DzBjGB0ACJAnASzBjOCyBEjBfnfnftJAnAVCfyB
ABC40BiAABAzADByB"####,
    ),
    (
        "regressions_loop_closure",
        r####"var x=0;function f(){for(x in {a:1}){};}f();x;
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBnABMAbyBn0ABLAnAjzBjYBfWzGiPjCjKjFjDjUCBzBjBDFdByBzAEf0DzBj
GF0ADJAnASByBndAftJAnAEjFfnfJAnAVBfyBABB40BiAABAEByB"####,
    ),
    (
        "regressions_loop_mixed",
        r####"var n=0;for(var x,y=[1,2],i=0;i<y.length;i++)n+=y[i];n;
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ADJAnASzBjOByBndAftKAJAnASByBCzBhLCnQzADfVzBjZEfyBVzBjJFf
yBnnntfARDVzBjYGfyBSEyBARCFdBFdCfnftSFyBndAfttCzBhcHVFfyBXzGjMjFjOjHjUjIIfVEfyB
nnTFyBBtJAnAVBfyBAEB40BiAG4B0AiAE4C0AiAF4D0AiAAEADByB"####,
    ),
    (
        "regressions_regexp_escape",
        r####"/[\\]+/.test("\\");
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ABJAnAEXzEjUjFjTjUBfYFibicicidhLARBFeBicff0DzACByB"####,
    ),
    (
        "regressions_regexp_shadow",
        r####"var RegExp=function(){throw "bad";}; /a/i.test("A");
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ACJAnASzGiSjFjHiFjYjQByBNyBnAMAbyBn0ABfAnAFeDjCjBjE0DzACC
AnftJAnAEXzEjUjFjTjUDfYBjBBjJRBFeBiBffABB40BiAABACByB"####,
    ),
    (
        "regressions_regexp_slash",
        r####"var x=/a\/b/;x.test("a/b");
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ACJAnASzBjYByBYEjBichPjCAnftJAnAEXzEjUjFjTjUCfVBfyBRBFeDj
BhPjCffABB40BiAABAzADByB"####,
    ),
    (
        "regressions_uninitialized",
        r####"var x; typeof x;
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ABJAnAizGjUjZjQjFjPjGBVzBjYCfyBABC40BiAABAzADByB"####,
    ),
    (
        "regressions_xml_ns_dynamic",
        r####"var ns=new Namespace("urn:x");var k="b";var x=<a xmlns:n="urn:x"><n:b>v</n:b></a>;x.ns::[k].toString();
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0AEJAnASzCjOjTByBEjzJiOjBjNjFjTjQjBjDjFCfRBFeFjVjSjOhajYft
nftJAnASzBjLDyBneBjCftJAnASzBjYEyBoBFehDhcjBhAjYjNjMjOjThajOhdhCjVjSjOhajYhCheh
cjOhajChejWhchPjOhajChehchPjBheAnftJAnAEXzIjUjPiTjUjSjJjOjHFfpBfVEfyBnVDfyBzAGn
fADE4C0AiAB40BiAD4B0AiAADAGByB"####,
    ),
    (
        "regressions_xml_plus_assign",
        r####"var x=<a/>;x+=<b/>;x.length();
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ADJAnASzBjYByBoBFeEhcjBhPheAnftJAnASByBCzBhLCnoBFeEhcjChP
heAnnntfJAnAEXzGjMjFjOjHjUjIDfVBfyBnfABB40BiAABAzAEByB"####,
    ),
    (
        "regressions_xml_type_variable",
        r####"var x=<a/>;typeof x;
"####,
        r####"@JSXBIN@ES@2.0@MyBbyBn0ACJAnASzBjYByBoBFeEhcjBhPheAnftJAnAizGjUjZjQjFjPjGCVBfyBA
BB40BiAABAzADByB"####,
    ),
    (
        "runtime_include_order",
        "var order=[];order.push(\"before\");order.push(\"included\");order.push(\"after\");order.join(\"|\");",
        "@JSXBIN@ES@2.0@MyBbyBn0AFJAnASzFjPjSjEjFjSByBAnnftJBnAEXzEjQjVjTjICfVBfyBRBFeGjC\njFjGjPjSjFffJhSnAEXCfVBfyBRBFeIjJjOjDjMjVjEjFjEffJDnAEXCfVBfyBRBFeFjBjGjUjFjSff\nJEnAEXzEjKjPjJjODfVBfyBRBFeBjcffABB40BiAABAzAEByB",
    ),
];
