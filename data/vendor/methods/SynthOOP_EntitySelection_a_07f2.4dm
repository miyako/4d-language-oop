// property read
// dynamic member: the doc's `.attributeName` is a name pattern, so a concrete member of the check project's own model stands in for it.
var $receiver : cs.SynthTableSelection
$receiver:=ds.SynthTable.all()
var $read1 : Collection
$read1:=$receiver.textValue
