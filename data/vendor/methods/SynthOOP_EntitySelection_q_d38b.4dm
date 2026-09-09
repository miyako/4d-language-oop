// property read
var $receiver : cs.SynthTableSelection
$receiver:=ds.SynthTable.all()
var $read1 : Text
$read1:=$receiver.queryPlan
// property write
$receiver:=ds.SynthTable.all()
$receiver.queryPlan:="synthText"
