// property read
var $receiver : 4D.SystemWorker
$receiver:=4D.SystemWorker.new("ls -l")
var $read1 : Text
$read1:=$receiver.responseError
// property write
$receiver:=4D.SystemWorker.new("ls -l")
$receiver.responseError:="synthText"
