// overload 0
var $receiver : 4D.SystemWorker
$receiver:=4D.SystemWorker.new("ls -l")
var $result1 : 4D.SystemWorker
$result1:=$receiver.wait(1)
// overload 0 [optional:omit-timeout]
$receiver:=4D.SystemWorker.new("ls -l")
var $result2 : 4D.SystemWorker
$result2:=$receiver.wait()
