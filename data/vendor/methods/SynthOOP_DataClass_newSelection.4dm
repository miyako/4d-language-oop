// overload 0
var $receiver : 4D.DataClass
$receiver:=ds.SynthTable
var $result1 : cs.SynthTableSelection
$result1:=$receiver.newSelection(dk keep ordered)
// overload 0 [enum DataClass.newSelection.keepOrder = dk non ordered]
$receiver:=ds.SynthTable
var $result2 : cs.SynthTableSelection
$result2:=$receiver.newSelection(dk non ordered)
// overload 0 [optional:omit-keepOrder]
$receiver:=ds.SynthTable
var $result3 : cs.SynthTableSelection
$result3:=$receiver.newSelection()
