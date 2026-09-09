// overload 0
var $receiver : cs.SynthTableSelection
$receiver:=ds.SynthTable.all()
var $result1 : cs.SynthTableSelection
$result1:=$receiver.slice(1; 1)
// overload 0 [optional:omit-end]
$receiver:=ds.SynthTable.all()
var $result2 : cs.SynthTableSelection
$result2:=$receiver.slice(1)
