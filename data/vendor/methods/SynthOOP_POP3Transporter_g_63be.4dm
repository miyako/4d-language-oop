// overload 0
var $receiver : 4D.POP3Transporter
$receiver:=4D.POP3Transporter.new(New object("host"; "pop.example.com"))
var $result1 : Collection
$result1:=$receiver.getMailInfoList()
