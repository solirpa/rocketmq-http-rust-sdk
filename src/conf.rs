
#[derive(Debug, Clone)]
pub struct ClientOption {
    pub endpoint: String,
    pub access_key_id: String,
    pub access_key_secret: String,
    pub security_token: Option<String>,
    pub namespace: Option<String>,
}

impl Default for ClientOption {
    fn default() -> Self {
        ClientOption {
            endpoint: "localhost:8081".to_string(),
            access_key_id: "".to_string(),
            access_key_secret: "".to_string(),
            security_token: None,
            namespace: None,
        }
    }
}

impl ClientOption {
    
    pub fn set_endpoint(&mut self, endpoint: impl Into<String>) {
        self.endpoint = endpoint.into();
    }

    pub fn get_endpoint(&self) -> &str {
        self.endpoint.as_str()
    }

    pub fn set_access_key_id(&mut self, access_key_id: impl Into<String>) {
        self.access_key_id = access_key_id.into();
    }

    pub fn get_access_key_id(&self) -> &str {
        self.access_key_id.as_str()
    }

    pub fn set_access_key_secret(&mut self, access_key_secret: impl Into<String>) {
        self.access_key_secret = access_key_secret.into();
    }

    pub fn get_access_key_secret(&self) -> &str {
        self.access_key_secret.as_str()
    }

    pub fn set_security_token(&mut self, security_token: impl Into<String>) {
        self.security_token = Some(security_token.into());
    }

    pub fn get_security_token(&self) -> Option<&str> {
        match &self.security_token {
            Some(token) => Some(token.as_str()),
            None => None,
        }
    }

    pub fn set_namespace(&mut self, namespace: impl Into<String>) {
        self.namespace = Some(namespace.into());
    }

    pub fn get_namespace(&self) -> Option<&str> {
        match &self.namespace {
            Some(namespace) => Some(namespace.as_str()),
            None => None,
        }
    }
}



/// The configuration of [`Consumer`].
#[derive(Debug, Clone)]
pub struct ConsumerOption {
    group: String,
    topic: String,
    tag: Option<String>,
}

impl Default for ConsumerOption {
    fn default() -> Self {
        ConsumerOption {
            group: "".to_string(),
            topic: "localhost:8081".to_string(),
            tag: None,
        }
    }
}

impl ConsumerOption {
    /// Create a new [`ConsumerOption`].
    pub fn set_group(&mut self, group: impl Into<String>) {
        self.group = group.into();
    }

    pub fn get_group(&self) -> &str {
        self.group.as_str()
    }

    pub fn set_topic(&mut self, topic: impl Into<String>) {
        self.topic = topic.into();
    }

    pub fn get_topic(&self) -> &str {
        self.topic.as_str()
    }

    pub fn set_tag(&mut self, tag: impl Into<String>) {
        self.tag = Some(tag.into());
    }

    pub fn get_tag(&self) -> Option<&str> {
        match &self.tag {
            Some(tag) => Some(tag.as_str()),
            None => None,
        }
    }
}


/// The configuration of [`Producer`].
#[derive(Debug, Clone)]
pub struct ProducerOption {
    group: Option<String>,
    topic: String,
}

impl Default for ProducerOption {
    fn default() -> Self {
        ProducerOption {
            group: None,
            topic: "".to_string(),
        }
    }
}

impl ProducerOption {
    /// Create a new [`ProducerOption`].
    pub fn set_group(&mut self, group: impl Into<String>) {
        self.group = Some(group.into());
    }

    pub fn get_group(&self) -> Option<&str> {
        match &self.group {
            Some(group) => Some(group.as_str()),
            None => None,
        }
    }

    pub fn set_topic(&mut self, topic: impl Into<String>) {
        self.topic = topic.into();
    }

    pub fn get_topic(&self) -> &str {
        self.topic.as_str()
    }
}