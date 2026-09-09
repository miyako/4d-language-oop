// overload 0
var $receiver : 4D.MailAttachment
$receiver:=4D.MailAttachment.new(File("/PACKAGE/report.pdf"))
var $result1 : 4D.Blob
$result1:=$receiver.getContent()
