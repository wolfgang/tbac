/*

LET C = 1
20 PRINT "HELLO"
LET C = C + 1
IF C > 10 GOTO 60
GOTO 20
60 PRINT "END"

 */

let counter = 0;
let running = true;

function goto(newValue) {
    counter = newValue
}

while (running) {
    switch (counter) {
        case 0:
            C = 1;
        case 20:
            console.log(C, "\tHELLO");
        case 0:
            C = C + 1;
        case 0:
            if (C > 10) {
                goto(60);
                break
            }
        case 0:
            goto(20);
            break;
        case 60:
            console.log("END");
        default:
            running = false;
    }
}