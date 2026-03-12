```
@startuml
actor User as U
participant "User Web UI" as UW
participant "Server" as S
participant "FileHandler" as FH

U -> UW: download file "filename"
activate UW
UW -> S: download_file("filename")
deactivate UW
activate S
S -> FH: handle_download("filename")
deactivate S
activate FH
FH -> S: path_to_file()
deactivate FH
activate S
S -> UW: file
deactivate S
activate UW
UW -> U: dowload file
deactivate UW
@enduml
```
