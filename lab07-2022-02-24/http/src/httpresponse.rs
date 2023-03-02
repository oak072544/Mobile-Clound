#[derive(Debug, PartialEq)]
pub enum StatusCode {
    Ok = 200,
    NotFound = 404,
    InternalServerError = 500,
}

impl StatusCode {
    fn reason_phrase(&self) -> &str {
        match self {
            StatusCode::Ok => "OK",
            StatusCode::NotFound => "Not Found",
            StatusCode::InternalServerError => "Internal Server Error",
        }
    }
}

pub struct HttpResponse<'a> {
    status_code: StatusCode,
    headers: Vec<(&'a str, &'a str)>,
    body: Option<&'a str>,
}

impl<'a> HttpResponse<'a> {
    pub fn new(status_code: StatusCode, headers: Vec<(&'a str, &'a str)>, body: Option<&'a str>) -> Self {
        HttpResponse { status_code, headers, body }
    }

    pub fn to_string(&self) -> String {
        let mut response = format!("HTTP/1.1 {} {}\r\n", self.status_code as u16, self.status_code.reason_phrase());
        for (name, value) in &self.headers {
            response += &format!("{}: {}\r\n", name, value);
        }
        if let Some(body) = self.body {
            response += &format!("Content-Length: {}\r\n\r\n{}", body.len(), body);
        } else {
            response += "\r\n";
        }
        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_string() {
        let headers = vec![("Content-Type", "text/plain"), ("Content-Language", "en-US")];
        let response = HttpResponse::new(StatusCode::Ok, headers, Some("Hello, world!"));
        let expected_output = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Language: en-US\r\nContent-Length: 13\r\n\r\nHello, world!".to_string();
        assert_eq!(response.to_string(), expected_output);
    }
}