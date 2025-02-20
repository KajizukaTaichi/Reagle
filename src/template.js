class ReagleObject {
    constructor(name) {
        this.value = name;
    }
    set(prop, value) {
        this[prop.value] = value;
        return this;
    }
    get(prop) {
        return this[prop.value];
    }
    puts() {
        console.log(this.value);
    }
}

class ReagleNumber extends ReagleObject {
    add(other) {
        return new ReagleNumber(this.value + other.value);
    }
    sub(other) {
        return new ReagleNumber(this.value - other.value);
    }
    mul(other) {
        return new ReagleNumber(this.value * other.value);
    }
    div(other) {
        return new ReagleNumber(this.value / other.value);
    }
    mod(other) {
        return new ReagleNumber(this.value % other.value);
    }
    pow(other) {
        return new ReagleNumber(this.value ** other.value);
    }
    eql(other) {
        return new ReagleBool(this.value == other.value);
    }
    repeat(block) {
        for (let i = 0; i < this.value; i++) {
            block();
        }
    }
}

class ReagleBool extends ReagleObject {
    and(other) {
        return new ReagleBool(this.value && other.value);
    }
    or(other) {
        return new ReagleBool(this.value || other.value);
    }
    not() {
        return new ReagleBool(!this.value);
    }
    then(block) {
        if (this.value) {
            block();
        }
        return this;
    }
    else(block) {
        if (this.value) {
            block();
        }
        return this;
    }
}

class ReagleString extends ReagleObject {
    add(other) {
        return new ReagleString(this.value + other.value);
    }
    mul(other) {
        return new ReagleString(this.value + other.value);
    }
    eql(other) {
        return new ReagleBool(this.value == other.value);
    }
}

let object = new ReagleObject(null);
let number = new ReagleNumber(0);
let string = new ReagleString("");
let bool = new ReagleBool(false);
