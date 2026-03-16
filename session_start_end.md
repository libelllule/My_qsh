```
@startuml
actor "User" as U
participant "User Web UI" as UW
participant "Server" as S
participant "BD_Handler" as DB

== Start Session ==

U -> UW: login
activate UW

UW -> S: request_session()
activate S

S -> DB: create_session(user_id)
activate DB
DB --> S: session_token
deactivate DB

S --> UW: session_token
deactivate S

UW -> U: show authorized state
deactivate UW


== End Session ==

U -> UW: logout
activate UW

UW -> S: end_session(session_token)
activate S

S -> DB: delete_session(user_id)
activate DB
DB --> S: ok
deactivate DB

S --> UW: session_closed
deactivate S

UW -> U: show logout message
deactivate UW

@enduml
```
