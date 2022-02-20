The code is structured in two important files. The com_processer and the com_executor.
The com_processer file is responsible for parsing the command and calling the
com_executor.
Solving this task I made the following assumptions:
    1) the dispute operation can work only on deposit transactions
    2) after an account is frozen it cannot be unfrozen by any other command

The commands are not all stored in the memory of the program, I keep the
position of each transaction, and when a dispute command appears,
I'm moving the iterator at that position and read it again.
For the charge_back and resolve operations I store the amount that has to be transactioned.
After a dispute is resolved or charged_back the dispute is canceled, and can not
be used withot another command.