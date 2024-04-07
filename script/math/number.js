class BasicType { }
class Radix10Integer extends BasicType {
	// the integer 1024 is create by `new Radix10Integer({ digits: [4, 2, 0, 1] })`
	static fromDefaultNumber(number) {
		return new Radix10Integer({ digits: number.toString().split('').map(c => parseInt(c)).reverse() });
	}
	static add(a, b) {
		let aLength = a.digits.length,
			bLength = b.digits.length;
		let [minDigitsNumber, maxDigitsNumber, maxDigitsList] = aLength < bLength ?
			[aLength, bLength, b.digits] :
			[bLength, aLength, a.digits];
		let newDigits = new Array(minDigitsNumber)
			.fill(0)
			.map((_, i) => a.digits[i] + b.digits[i]);
		newDigits.push(...maxDigitsList.splice(minDigitsNumber, maxDigitsNumber));
		newDigits.map((n, i) => {
			if (n >= 10) {
				if (newDigits[i + 1] === undefined) {
					newDigits[i + 1] = 0;
				}
				newDigits[i + 1] += Math.floor(n / 10);
				newDigits[i] = n % 10;
			}
		});
		return new Radix10Integer({ digits: newDigits });
	}
	constructor({ digits = [0] }) {
		super();
		this.digits = digits;
	}
}

class AdvancedType { }
class Number extends AdvancedType { }
class Integer extends Number {
	static add(a, b) {

	}
	constructor({
		radix = Radix10Integer.fromDefaultNumber(10),
		digits = [Radix10Integer.fromDefaultNumber(0)]
	}) {
		super();
		this.radix = radix;
		this.digits = digits;
	}
	asRadix(radix) {
		
	}
	convert(targetType) {
		switch (targetType) {
			case Fraction:
				return new Fraction({ numerator: this.value, denominator: 1 });
		}
	}
}
class Fraction extends Number {
	constructor({ numerator, denominator }) {
		this.numerator = numerator;
		this.denominator = denominator;
	}
	add() {

	}
}