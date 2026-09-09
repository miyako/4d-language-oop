// overload 0
var $result1 : 4D.File
$result1:=4D.File.new("synthText"; fk platform path)
// overload 0 [enum 4D.File.new.pathType = fk posix path]
var $result2 : 4D.File
$result2:=4D.File.new("synthText"; fk posix path)
// overload 0 [optional:omit-pathType]
var $result3 : 4D.File
$result3:=4D.File.new("synthText")
// overload 1
var $result4 : 4D.File
$result4:=4D.File.new(fk applications folder)
// overload 1 [enum 4D.File.new.fileConstant = fk data folder]
var $result5 : 4D.File
$result5:=4D.File.new(fk data folder)
// overload 1 [enum 4D.File.new.fileConstant = fk database folder]
var $result6 : 4D.File
$result6:=4D.File.new(fk database folder)
// overload 1 [enum 4D.File.new.fileConstant = fk desktop folder]
var $result7 : 4D.File
$result7:=4D.File.new(fk desktop folder)
// overload 1 [enum 4D.File.new.fileConstant = fk documents folder]
var $result8 : 4D.File
$result8:=4D.File.new(fk documents folder)
// overload 1 [enum 4D.File.new.fileConstant = fk editor theme folder]
var $result9 : 4D.File
$result9:=4D.File.new(fk editor theme folder)
// overload 1 [enum 4D.File.new.fileConstant = fk home folder]
var $result10 : 4D.File
$result10:=4D.File.new(fk home folder)
// overload 1 [enum 4D.File.new.fileConstant = fk licenses folder]
var $result11 : 4D.File
$result11:=4D.File.new(fk licenses folder)
// overload 1 [enum 4D.File.new.fileConstant = fk logs folder]
var $result12 : 4D.File
$result12:=4D.File.new(fk logs folder)
// overload 1 [enum 4D.File.new.fileConstant = fk mobileApps folder]
var $result13 : 4D.File
$result13:=4D.File.new(fk mobileApps folder)
// overload 1 [enum 4D.File.new.fileConstant = fk remote database folder]
var $result14 : 4D.File
$result14:=4D.File.new(fk remote database folder)
// overload 1 [enum 4D.File.new.fileConstant = fk resources folder]
var $result15 : 4D.File
$result15:=4D.File.new(fk resources folder)
// overload 1 [enum 4D.File.new.fileConstant = fk system folder]
var $result16 : 4D.File
$result16:=4D.File.new(fk system folder)
// overload 1 [enum 4D.File.new.fileConstant = fk user preferences folder]
var $result17 : 4D.File
$result17:=4D.File.new(fk user preferences folder)
// overload 1 [enum 4D.File.new.fileConstant = fk web root folder]
var $result18 : 4D.File
$result18:=4D.File.new(fk web root folder)
