// overload 0
var $receiver : cs.SynthTableSelection
$receiver:=ds.SynthTable.all()
var $selectedEntities1 : cs.SynthTableSelection
$selectedEntities1:=ds.SynthTable.all()
var $result2 : Object
$result2:=$receiver.selected($selectedEntities1)
