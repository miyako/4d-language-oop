// overload 0
var $receiver : 4D.Vector
$receiver:=4D.Vector.new([0.1; 0.2; 0.3])
var $vector1 : 4D.Vector
$vector1:=4D.Vector.new([0.1; 0.2; 0.3])
var $result2 : Real
$result2:=$receiver.euclideanDistance($vector1)
