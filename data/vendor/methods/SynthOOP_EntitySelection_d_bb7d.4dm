// overload 0
var $receiver : cs.SynthTableSelection
$receiver:=ds.SynthTable.all()
var $result1 : Collection
$result1:=$receiver.distinctPaths("synthText")
