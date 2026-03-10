use super::super::{ParseError, extract};

pub(crate) fn parse_memorypack_i32(payload: &[u8]) -> Result<i32, ParseError> {
    if payload.len() < 4 {
        return Err(ParseError::InvalidPayload {
            expected: 4,
            actual: payload.len(),
        });
    }

    Ok(i32::from_le_bytes(payload[0..4].try_into().map_err(
        |_| ParseError::InvalidPayload {
            expected: 4,
            actual: payload.len(),
        },
    )?))
}

pub(crate) fn parse_memorypack_i64(payload: &[u8]) -> Result<i64, ParseError> {
    if payload.len() < 8 {
        return Err(ParseError::InvalidPayload {
            expected: 8,
            actual: payload.len(),
        });
    }

    Ok(i64::from_le_bytes(payload[0..8].try_into().map_err(
        |_| ParseError::InvalidPayload {
            expected: 8,
            actual: payload.len(),
        },
    )?))
}

pub(crate) fn parse_memorypack_f32(payload: &[u8]) -> Result<f32, ParseError> {
    if payload.len() < 4 {
        return Err(ParseError::InvalidPayload {
            expected: 4,
            actual: payload.len(),
        });
    }

    Ok(f32::from_le_bytes(payload[0..4].try_into().map_err(
        |_| ParseError::InvalidPayload {
            expected: 4,
            actual: payload.len(),
        },
    )?))
}

pub(crate) fn parse_memorypack_f64(payload: &[u8]) -> Result<f64, ParseError> {
    if payload.len() < 8 {
        return Err(ParseError::InvalidPayload {
            expected: 8,
            actual: payload.len(),
        });
    }

    Ok(f64::from_le_bytes(payload[0..8].try_into().map_err(
        |_| ParseError::InvalidPayload {
            expected: 8,
            actual: payload.len(),
        },
    )?))
}

pub(crate) fn parse_memorypack_u8(payload: &[u8]) -> Result<u8, ParseError> {
    if payload.is_empty() {
        return Err(ParseError::InvalidPayload {
            expected: 1,
            actual: 0,
        });
    }

    Ok(payload[0])
}

pub(crate) fn parse_memorypack_u32(payload: &[u8]) -> Result<u32, ParseError> {
    if payload.len() < 4 {
        return Err(ParseError::InvalidPayload {
            expected: 4,
            actual: payload.len(),
        });
    }

    Ok(u32::from_le_bytes(payload[0..4].try_into().map_err(
        |_| ParseError::InvalidPayload {
            expected: 4,
            actual: payload.len(),
        },
    )?))
}

pub(crate) fn parse_memorypack_vector3(payload: &[u8]) -> Result<extract::Vector3, ParseError> {
    if payload.len() < 12 {
        return Err(ParseError::InvalidPayload {
            expected: 12,
            actual: payload.len(),
        });
    }
    Ok(extract::Vector3 {
        x: f32::from_le_bytes(payload[0..4].try_into().map_err(|_| {
            ParseError::InvalidPayload {
                expected: 4,
                actual: payload.len(),
            }
        })?),
        y: f32::from_le_bytes(payload[4..8].try_into().map_err(|_| {
            ParseError::InvalidPayload {
                expected: 8,
                actual: payload.len(),
            }
        })?),
        z: f32::from_le_bytes(payload[8..12].try_into().map_err(|_| {
            ParseError::InvalidPayload {
                expected: 12,
                actual: payload.len(),
            }
        })?),
    })
}

pub(crate) fn parse_memorypack_quaternion(
    payload: &[u8],
) -> Result<extract::Quaternion, ParseError> {
    if payload.len() < 16 {
        return Err(ParseError::InvalidPayload {
            expected: 16,
            actual: payload.len(),
        });
    }
    Ok(extract::Quaternion {
        x: f32::from_le_bytes(payload[0..4].try_into().map_err(|_| {
            ParseError::InvalidPayload {
                expected: 4,
                actual: payload.len(),
            }
        })?),
        y: f32::from_le_bytes(payload[4..8].try_into().map_err(|_| {
            ParseError::InvalidPayload {
                expected: 8,
                actual: payload.len(),
            }
        })?),
        z: f32::from_le_bytes(payload[8..12].try_into().map_err(|_| {
            ParseError::InvalidPayload {
                expected: 12,
                actual: payload.len(),
            }
        })?),
        w: f32::from_le_bytes(payload[12..16].try_into().map_err(|_| {
            ParseError::InvalidPayload {
                expected: 16,
                actual: payload.len(),
            }
        })?),
    })
}

pub(crate) fn parse_memorypack_bool(payload: &[u8]) -> Result<bool, ParseError> {
    if payload.is_empty() {
        return Err(ParseError::InvalidPayload {
            expected: 1,
            actual: 0,
        });
    }
    Ok(payload[0] != 0)
}

pub(crate) fn parse_memorypack_string_with_len(payload: &[u8]) -> Result<(String, usize), ParseError> {
    if payload.len() < 4 {
        return Err(ParseError::InvalidPayload {
            expected: 4,
            actual: payload.len(),
        });
    }

    let marker =
        i32::from_le_bytes(
            payload[0..4]
                .try_into()
                .map_err(|_| ParseError::InvalidPayload {
                    expected: 4,
                    actual: payload.len(),
                })?,
        );

    if marker == -1 || marker == 0 {
        return Ok((String::new(), 4));
    }

    if marker > 0 {
        let utf16_units = marker as usize;
        let bytes_len = utf16_units * 2;
        let expected = 4 + bytes_len;
        if payload.len() < expected {
            return Err(ParseError::InvalidPayload {
                expected,
                actual: payload.len(),
            });
        }

        let mut units = Vec::with_capacity(utf16_units);
        for chunk in payload[4..expected].chunks_exact(2) {
            units.push(u16::from_le_bytes([chunk[0], chunk[1]]));
        }
        return Ok((String::from_utf16_lossy(&units), expected));
    }

    if payload.len() < 8 {
        return Err(ParseError::InvalidPayload {
            expected: 8,
            actual: payload.len(),
        });
    }

    let utf8_len = (!marker) as usize;
    let expected = 8 + utf8_len;
    if payload.len() < expected {
        return Err(ParseError::InvalidPayload {
            expected,
            actual: payload.len(),
        });
    }

    Ok((
        String::from_utf8_lossy(&payload[8..expected]).to_string(),
        expected,
    ))
}

pub(crate) fn parse_memorypack_string(payload: &[u8]) -> Result<String, ParseError> {
    parse_memorypack_string_with_len(payload).map(|(s, _)| s)
}
