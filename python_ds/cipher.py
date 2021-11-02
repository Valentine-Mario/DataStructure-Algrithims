class CaesarCipher:
    def __init__(self, shift):
        encoder=[None]*26 #tmp arr for enctyption
        decoder=[None]*26 #tmp arr for decoding
        for k in range(26):
            #get char string for index
            encoder[k]=chr((k+shift)%26 + ord('A'))
            decoder[k]=chr((k+shift)%26 + ord('A'))
        self._forward=''.join(encoder)
        self._backward=''.join(decoder)

    def _transform(self, original, code):
        msg=list(original)
        for k in range(len(msg)):
            if msg[k].isupper():
                j=ord(msg[k])-ord('A')
                msg[k]=code[j]
        return ''.join(msg)

    def encrypt(self, message):
        return self._transform(message, self._forward)
    
    def decrypt(self, message):
        return self._transform(message, self._backward)


cipher=CaesarCipher(3)
msg="THIS IS A TEXT"
enc=cipher.encrypt(msg)
print(enc)
print(cipher.decrypt(enc))