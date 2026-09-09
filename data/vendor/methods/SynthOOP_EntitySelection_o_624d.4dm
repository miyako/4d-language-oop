// overload 0
var $receiver : cs.SynthTableSelection
$receiver:=ds.SynthTable.all()
var $result1 : cs.SynthTableSelection
$result1:=$receiver.orderBy("synthText")
// overload 1
$receiver:=ds.SynthTable.all()
var $result2 : cs.SynthTableSelection
$result2:=$receiver.orderBy(New collection)
