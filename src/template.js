class ReagleObject {
    constructor(name) {
        this.value = name;
    }
    set(prop, value) {
        this[prop] = value;
        return this;
    }
    get(prop) {
        return this[prop];
    }
    puts() {
        console.log(this.value);
    }
}

class ReagleNumber extends ReagleObject {
    add(other) {
        return ReagleNumber(this.value + other.value);
    }
    sub(other) {
        return ReagleNumber(this.value - other.value);
    }
    mul(other) {
        return ReagleNumber(this.value * other.value);
    }
    div(other) {
        return ReagleNumber(this.value / other.value);
    }
    mod(other) {
        return ReagleNumber(this.value % other.value);
    }
    pow(other) {
        return ReagleNumber(this.value ** other.value);
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
        return ReagleNumber(this.value && other.value);
    }
    or(other) {
        return ReagleNumber(this.value || other.value);
    }
    not() {
        return ReagleNumber(!this.value);
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
        this.value += other.value;
        return this;
    }
    mul(other) {
        this.value = this.value.repeat(other.value);
        return this;
    }
    eql(other) {
        return new ReagleBool(this.value == other.value);
    }
}
