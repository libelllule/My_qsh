```
@startuml
interface UI {
  + handle_request(req: Request): Response
  + render_static_page(page_name: &str): Response
}

class ClientUI {
  - state: Arc<AppState>
  + handle_index(req: Request): Response
  + handle_upload(req: Request): Response
  + handle_download(req: Request): Response
  + handle_list(req: Request): Response
  + handle_delete(req: Request): Response
  + handle_login(req: Request): Response
  + handle_logout(req: Request): Response
}

class ServerUI {
  - state: Arc<AppState>
  + handle_admin_index(req: Request): Response
  + handle_users_list(req: Request): Response
  + handle_user_approve(req: Request): Response
  + handle_user_reject(req: Request): Response
  + handle_user_remove(req: Request): Response
  + handle_add_user(req: Request): Response
  + handle_server_status(req: Request): Response
  + handle_server_shutdown(req: Request): Response
  + handle_files_list_admin(req: Request): Response
  + handle_file_delete_admin(req: Request): Response
}

class ParamHandler {
  - config: Config
  + new(): Self
  + parse_args(): Result<Config, Error>
  + get_config(): Config
}

class FileHandler {
  - storage_path: PathBuf
  + new(storage_path: PathBuf): Self
  + save_file(file_name: &str, data: &[u8], user_id: Option<&str>): Result<String, Error>
  + get_file(file_id: &str): Result<Vec<u8>, Error>
  + delete_file(file_id: &str, user_id: Option<&str>): Result<(), Error>
  + list_files(user_id: Option<&str>): Result<Vec<FileInfo>, Error>
  + get_metadata(file_id: &str): Result<FileInfo, Error>
}

class Config {
  ' настройки сервера (порт, пути к сертификатам, строка подключения к БД, папка для файлов и т.д.)
}

title Classes diagram QSH
class "BDHandler" as bd {
  -String db_url
  -Pool<Sqlite> pool
  -String initializate_filename

  +initialize_db(&self) : u8
  +create_tables(&self) : u8
  +clear_db(&self) : u8
  +add_users(&self, users: Vec<user: User>) : u8
  +add_notes(&self, notes: Vec<note: Note>) : u8
  +update_user(&self, old_user: User, new_user: User) : u8
  +update_note(&self, old_note: Note, new_note: Note) : u8
  +delete_user(&self, user: User) : u8
  +delete_note(&self, note: Note) : u8
  +execute_query(&self, query: String) : String
}

class "User" as usr {
  -u64 id
  +MacAddress mac_addr
  +String device_name
  +String nickname
  +String status

  +to_string(&self) : String
  +get_mac(&self) : String
  +get_device_name(&self) : String
  +get_nickname(&self) : String
  +get_status(&self) : u8
}

class "Note" as nt {
  -u64 id
  -u64 user_id
  +String filename
  +String size
  +String date
  +String status

  +to_string(&self) : String
  +get_status(&self) : u8
}

class "Security" as sec {
  -String path_to_certs
  +generate_certs(&self, path: String) : u8
  +validate_certs(&self) : u8
}

class "ConnectionHandler" as conhndlr {
  -User user
  -String user_ip
  -MacAddres mac_addr
  -String user_device_name
  
  +get_mac(&self) : String
  +get_ip(&self) : String
  +get_device_name(&self) : String
  +check_user(&self, user: User, ip: String) : u8
  +create_user(&self) : User
  +approve_user(&self) : u8
  +block_user(&self) : u8
  +deny_user(&self) : u8
  +send_approval_code(&self) : u8
}

class "Server" as srv {
  -ParamHandler param_handler
  -ClientUI client_ui
  -ServerUI server_ui
  -FileHandler file_handler
  -BDHandler BD_handler
  -ConnectionHandler connection_handler
  -Security Security
  -HashMap parameters
  +init(&self)
  +main(&self)
}


srv -- bd
srv --conhndlr
srv -- sec
usr -- conhndlr
srv -- ParamHandler
srv -- FileHandler
srv -- ClientUI
srv -- ServerUI
ClientUI -- FileHandler
ServerUI -- FileHandler

ClientUI ..|> UI
ServerUI ..|> UI
ParamHandler --> Config
@enduml
```
