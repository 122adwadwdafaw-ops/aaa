// Original ScriptUI state fixture, @cdinhoo, MIT OR Apache-2.0.
// Safe to run in either host: it changes no project or preferences.
(function () {
    var w = new Window('palette', 'ScriptUI State Reference');
    w.orientation = 'column'; w.alignChildren = ['fill', 'top']; w.spacing = 8; w.margins = 14;
    var normal = w.add('button', undefined, 'Normal button', {name:'normal'});
    var custom = w.add('button', undefined, 'Custom state', {name:'custom'});
    custom.preferredSize = [240, 34];
    custom.onDraw = function (state) {
        var g = this.graphics;
        var color = [0.2,0.35,0.65,1];
        if (!this.enabled) color = [0.25,0.25,0.25,1];
        else if (state.leftButtonPressed) color = [0.75,0.25,0.2,1];
        else if (state.mouseOver) color = [0.25,0.6,0.35,1];
        g.newPath(); g.rectPath(0,0,this.size.width,this.size.height); g.fillPath(g.newBrush(g.BrushType.SOLID_COLOR,color));
        g.drawString(this.text,g.newPen(g.PenType.SOLID_COLOR,[1,1,1,1],1),12,9);
    };
    var checks = w.add('group');
    checks.add('checkbox',undefined,'Enabled option',{name:'check'}).value = true;
    checks.add('radiobutton',undefined,'Selected',{name:'radio'}).value = true;
    var disabled = w.add('group'); disabled.orientation = 'column'; disabled.alignChildren = ['fill','top'];
    disabled.add('button',undefined,'Disabled parent button',{name:'disabled'});
    disabled.add('checkbox',undefined,'Disabled option').value = true;
    disabled.enabled = false;
    var tabs = w.add('tabbedpanel'); tabs.preferredSize = [260,85];
    tabs.add('tab',undefined,'First').add('statictext',undefined,'Selected tab');
    tabs.add('tab',undefined,'Second').add('statictext',undefined,'Second tab content');
    var off = tabs.add('tab',undefined,'Disabled'); off.enabled = false;
    $.global.__stateReference = w;
    w.show();
})();
