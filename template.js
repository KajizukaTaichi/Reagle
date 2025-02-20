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
    println() {
        console.log(this.value);
    }
}

class ReagleNumber extends ReagleObject {
    add(other) {
        this.value += other.value;
        return this;
    }
    sub(other) {
        this.value -= other.value;
        return this;
    }
    mul(other) {
        this.value *= other.value;
        return this;
    }
    div(other) {
        this.value /= other.value;
        return this;
    }
    eql(other) {
        return new ReagleBool(this.value == other.value);
    }
    repeat(block) {
        for (let i = 0; i < this.value; i++) {
            block(new ReagleNumber(i));
        }
    }
}

class ReagleBool extends ReagleObject {
    and(other) {
        this.value &&= other.value;
        return this;
    }
    or(other) {
        this.value ||= other.value;
        return this;
    }
    not() {
        this.value = !this.value;
        return this;
    }
    if(cond, block) {
        if (this.value === cond.value) {
            block();
        }
        return this;
    }
}

new ReagleNumber(5).repeat(function (...args) {
    return args[0].println();
});
