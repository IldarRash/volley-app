package com.courte.booking.exceptions

class InvalidRequestException(val subject: String, val violation: String) : RuntimeException("$subject: $violation")