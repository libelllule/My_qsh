@startuml
actor User as U
participant "User Web UI" as UW
participant "Server" as S
participant "FileHandler" as FH
participant "BD_Handler" as DB

U -> UW: upload file "filename"
activate UW 

UW -> S: upload_file("filename", data) 
activate S

S -> FH: handle_upload("filename", data)
activate FH
FH --> S: upload_status (ok)
deactivate FH

S -> DB: log_event(user_id, filename, size, 'u')
activate DB
DB --> S: ok
deactivate DB

S --> UW: success_response
deactivate S

UW -> U: show success message
deactivate UW
@enduml
