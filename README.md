CMD_TASK

CMD_Task is lil cli task management application I'm currently building.
It currently only has ability to add and list existing tasks. It writes all data pertaining the tasks to a JSON file. 
I am leveraging clap for the commandline framework, and serde to serailize and deserialize structs to JSON. 

TODO:
Deletion of tasks.
Automated assignment of task ID's.
Checking tasks by due date. 
Custom JSON db location. 
