// property read
var $receiver : cs.SynthTableSelection
$receiver:=ds.SynthTable.all()
var $read1 : Integer
$read1:=$receiver.length
// property write
$receiver:=ds.SynthTable.all()
$receiver.length:=1
