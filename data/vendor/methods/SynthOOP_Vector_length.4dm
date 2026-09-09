// property read
var $receiver : 4D.Vector
$receiver:=4D.Vector.new([0.1; 0.2; 0.3])
var $read1 : Integer
$read1:=$receiver.length
// property write
$receiver:=4D.Vector.new([0.1; 0.2; 0.3])
$receiver.length:=1
