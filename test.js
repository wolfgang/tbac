/*

10 LET C = 1
20 PRINT "HELLO"
30 LET C = C + 1
40 IF C > 10 GOTO 60
50 GOTO 20
60 PRINT "END"

10 PRINT "HELLO"

 */

let counter = 10;

function goto(newValue) {
    counter = newValue
}

let running = true;

while (running) {
    switch (counter) {
        case 10:
            C = 1;
        case 20:
            console.log(C, "\tHELLO");
        case 30:
            C = C + 1;
        case 40:
            if (C > 10) {
                goto(60);
                break
            }
        case 50:
            goto(20);
            break;
        case 60:
            console.log("END");
        default:
            running = false;
    }
}