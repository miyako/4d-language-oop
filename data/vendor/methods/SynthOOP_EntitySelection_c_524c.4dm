// overload 0
var $receiver : cs.SynthTableSelection
$receiver:=ds.SynthTable.all()
var $entity1 : cs.SynthTableEntity
$entity1:=ds.SynthTable.new()
var $result2 : Boolean
$result2:=$receiver.contains($entity1)
