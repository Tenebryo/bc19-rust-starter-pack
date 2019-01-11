import {BCAbstractRobot, SPECS} from 'battlecode'
import {test_fn, test_turn} from './bot/native.js'

class MyRobot extends BCAbstractRobot {

    turn() {
        this.log(test_fn(12.0, 5.0))
        return test_turn(this);
    }
}