```
@startuml
actor User as U
participant "User Web UI" as UW
participant "Server" as S
participant "FileHandler" as FH
participant "BD_Handler" as DB

U -> UW: download file "filename"
activate UW

UW -> S: download_file("filename")
activate S

S -> FH: handle_download("filename")
activate FH
FH --> S: file_data (stream)
deactivate FH

S -> DB: log_event(user_id, filename, 'd')
activate DB
DB --> S: ok
deactivate DB

S --> UW: file
deactivate S

UW -> U: save file
deactivate UW
@enduml

```
