@startuml
actor "Host" as H
participant "Host Web UI" as HW
participant "Server" as S
participant "BD_Handler" as DB

== Block a user (/block/) ==
H -> HW: select user and click "Block"
activate HW
HW -> S: block_user(user_id, session_token)
activate S
S -> DB: set_user_status(user_id, 'b') 
activate DB
DB --> S: ok
deactivate DB
S --> HW: user_blocked_confirm
deactivate S
deactivate HW

== Add a trusted user (/trust/) ==
H -> HW: select user and click "Trust"
activate HW
HW -> S: trust_user(user_id, session_token) 
activate S
S -> DB: set_user_status(user_id, 't')
activate DB
DB --> S: ok
deactivate DB
S --> HW: user_trusted_confirm
deactivate S
deactivate HW
@enduml