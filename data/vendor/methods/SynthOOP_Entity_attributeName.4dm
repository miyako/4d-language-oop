// property read
// dynamic member: the doc's `.attributeName` is a name pattern, so a concrete member of the check project's own model stands in for it.
var $receiver : cs.SynthTableEntity
$receiver:=ds.SynthTable.new()
var $read1 : Text
$read1:=$receiver.textValue
