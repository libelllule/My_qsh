@startuml
actor "User" as U
participant "User Web UI" as UW
participant "Server" as S
participant "BD_Handler" as DB
participant "Host Web UI" as HW

U -> UW: request access (smileys)
activate UW

UW -> S: send_join_request(smileys) 
activate S
 
S -> DB: get_user_status(user_id)
activate DB
DB --> S: status ('t', 'b', or 'u') 
deactivate DB

alt Status is 't' (Trusted) 
    S --> UW: access_granted (token)  
    UW -> U: show file list 

else Status is 'b' (Blocked)
    S --> UW: access_denied (auto)
    UW -> U: show "Access Denied"

else Status is 'u' (Unknown) 
    S -> HW: notify_new_request(smileys)
    activate HW
    
    alt Host manually accepts (/accept/)
        HW -> S: approve_user
        S --> UW: access_granted (token)
        UW -> U: show file list
    else Host manually denies (/deny/)
        HW -> S: deny_user
        S --> UW: access_denied
        UW -> U: show "Access Denied"
    end
    deactivate HW
end

deactivate S
deactivate UW
@enduml
