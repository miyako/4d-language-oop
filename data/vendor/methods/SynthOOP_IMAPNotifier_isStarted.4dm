// property read
var $receiver : 4D.IMAPNotifier
$receiver:=4D.IMAPNotifier.new()
var $read1 : Boolean
$read1:=$receiver.isStarted
