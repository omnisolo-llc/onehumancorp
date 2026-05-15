pub mod mesh_handler;
pub mod autodream;

pub mod billing_webhook;
#[cfg(test)]
pub mod billing_webhook_test;
pub mod health;
pub mod agents;
pub mod onboarding;
pub mod growth;

#[allow(dead_code)]
pub const OHC_API_SCHEMA: &str = r#"
{
    "openapi": "3.0.0",
    "info": {
        "title": "OHC Platform API",
        "version": "1.0.0",
        "description": "Comprehensive API schema for all internal services"
    },
    "paths": {
        "/api/v1/resource_1": {
            "get": {
                "summary": "Get Resource 1",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 1",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_2": {
            "get": {
                "summary": "Get Resource 2",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 2",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_3": {
            "get": {
                "summary": "Get Resource 3",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 3",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_4": {
            "get": {
                "summary": "Get Resource 4",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 4",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_5": {
            "get": {
                "summary": "Get Resource 5",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 5",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_6": {
            "get": {
                "summary": "Get Resource 6",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 6",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_7": {
            "get": {
                "summary": "Get Resource 7",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 7",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_8": {
            "get": {
                "summary": "Get Resource 8",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 8",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_9": {
            "get": {
                "summary": "Get Resource 9",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 9",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_10": {
            "get": {
                "summary": "Get Resource 10",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 10",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_11": {
            "get": {
                "summary": "Get Resource 11",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 11",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_12": {
            "get": {
                "summary": "Get Resource 12",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 12",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_13": {
            "get": {
                "summary": "Get Resource 13",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 13",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_14": {
            "get": {
                "summary": "Get Resource 14",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 14",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_15": {
            "get": {
                "summary": "Get Resource 15",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 15",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_16": {
            "get": {
                "summary": "Get Resource 16",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 16",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_17": {
            "get": {
                "summary": "Get Resource 17",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 17",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_18": {
            "get": {
                "summary": "Get Resource 18",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 18",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_19": {
            "get": {
                "summary": "Get Resource 19",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 19",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_20": {
            "get": {
                "summary": "Get Resource 20",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 20",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_21": {
            "get": {
                "summary": "Get Resource 21",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 21",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_22": {
            "get": {
                "summary": "Get Resource 22",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 22",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_23": {
            "get": {
                "summary": "Get Resource 23",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 23",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_24": {
            "get": {
                "summary": "Get Resource 24",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 24",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_25": {
            "get": {
                "summary": "Get Resource 25",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 25",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_26": {
            "get": {
                "summary": "Get Resource 26",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 26",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_27": {
            "get": {
                "summary": "Get Resource 27",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 27",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_28": {
            "get": {
                "summary": "Get Resource 28",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 28",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_29": {
            "get": {
                "summary": "Get Resource 29",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 29",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_30": {
            "get": {
                "summary": "Get Resource 30",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 30",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_31": {
            "get": {
                "summary": "Get Resource 31",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 31",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_32": {
            "get": {
                "summary": "Get Resource 32",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 32",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_33": {
            "get": {
                "summary": "Get Resource 33",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 33",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_34": {
            "get": {
                "summary": "Get Resource 34",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 34",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_35": {
            "get": {
                "summary": "Get Resource 35",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 35",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_36": {
            "get": {
                "summary": "Get Resource 36",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 36",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_37": {
            "get": {
                "summary": "Get Resource 37",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 37",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_38": {
            "get": {
                "summary": "Get Resource 38",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 38",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_39": {
            "get": {
                "summary": "Get Resource 39",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 39",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_40": {
            "get": {
                "summary": "Get Resource 40",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 40",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_41": {
            "get": {
                "summary": "Get Resource 41",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 41",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_42": {
            "get": {
                "summary": "Get Resource 42",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 42",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_43": {
            "get": {
                "summary": "Get Resource 43",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 43",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_44": {
            "get": {
                "summary": "Get Resource 44",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 44",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_45": {
            "get": {
                "summary": "Get Resource 45",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 45",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_46": {
            "get": {
                "summary": "Get Resource 46",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 46",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_47": {
            "get": {
                "summary": "Get Resource 47",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 47",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_48": {
            "get": {
                "summary": "Get Resource 48",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 48",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_49": {
            "get": {
                "summary": "Get Resource 49",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 49",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_50": {
            "get": {
                "summary": "Get Resource 50",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 50",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_51": {
            "get": {
                "summary": "Get Resource 51",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 51",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_52": {
            "get": {
                "summary": "Get Resource 52",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 52",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_53": {
            "get": {
                "summary": "Get Resource 53",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 53",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_54": {
            "get": {
                "summary": "Get Resource 54",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 54",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_55": {
            "get": {
                "summary": "Get Resource 55",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 55",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_56": {
            "get": {
                "summary": "Get Resource 56",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 56",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_57": {
            "get": {
                "summary": "Get Resource 57",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 57",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_58": {
            "get": {
                "summary": "Get Resource 58",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 58",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_59": {
            "get": {
                "summary": "Get Resource 59",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 59",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_60": {
            "get": {
                "summary": "Get Resource 60",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 60",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_61": {
            "get": {
                "summary": "Get Resource 61",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 61",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_62": {
            "get": {
                "summary": "Get Resource 62",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 62",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_63": {
            "get": {
                "summary": "Get Resource 63",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 63",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_64": {
            "get": {
                "summary": "Get Resource 64",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 64",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_65": {
            "get": {
                "summary": "Get Resource 65",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 65",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_66": {
            "get": {
                "summary": "Get Resource 66",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 66",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_67": {
            "get": {
                "summary": "Get Resource 67",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 67",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_68": {
            "get": {
                "summary": "Get Resource 68",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 68",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_69": {
            "get": {
                "summary": "Get Resource 69",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 69",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_70": {
            "get": {
                "summary": "Get Resource 70",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 70",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_71": {
            "get": {
                "summary": "Get Resource 71",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 71",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_72": {
            "get": {
                "summary": "Get Resource 72",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 72",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_73": {
            "get": {
                "summary": "Get Resource 73",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 73",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_74": {
            "get": {
                "summary": "Get Resource 74",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 74",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_75": {
            "get": {
                "summary": "Get Resource 75",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 75",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_76": {
            "get": {
                "summary": "Get Resource 76",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 76",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_77": {
            "get": {
                "summary": "Get Resource 77",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 77",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_78": {
            "get": {
                "summary": "Get Resource 78",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 78",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_79": {
            "get": {
                "summary": "Get Resource 79",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 79",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_80": {
            "get": {
                "summary": "Get Resource 80",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 80",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_81": {
            "get": {
                "summary": "Get Resource 81",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 81",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_82": {
            "get": {
                "summary": "Get Resource 82",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 82",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_83": {
            "get": {
                "summary": "Get Resource 83",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 83",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_84": {
            "get": {
                "summary": "Get Resource 84",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 84",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_85": {
            "get": {
                "summary": "Get Resource 85",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 85",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_86": {
            "get": {
                "summary": "Get Resource 86",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 86",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_87": {
            "get": {
                "summary": "Get Resource 87",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 87",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_88": {
            "get": {
                "summary": "Get Resource 88",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 88",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_89": {
            "get": {
                "summary": "Get Resource 89",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 89",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_90": {
            "get": {
                "summary": "Get Resource 90",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 90",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_91": {
            "get": {
                "summary": "Get Resource 91",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 91",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_92": {
            "get": {
                "summary": "Get Resource 92",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 92",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_93": {
            "get": {
                "summary": "Get Resource 93",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 93",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_94": {
            "get": {
                "summary": "Get Resource 94",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 94",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_95": {
            "get": {
                "summary": "Get Resource 95",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 95",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_96": {
            "get": {
                "summary": "Get Resource 96",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 96",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_97": {
            "get": {
                "summary": "Get Resource 97",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 97",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_98": {
            "get": {
                "summary": "Get Resource 98",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 98",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_99": {
            "get": {
                "summary": "Get Resource 99",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 99",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_100": {
            "get": {
                "summary": "Get Resource 100",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 100",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_101": {
            "get": {
                "summary": "Get Resource 101",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 101",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_102": {
            "get": {
                "summary": "Get Resource 102",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 102",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_103": {
            "get": {
                "summary": "Get Resource 103",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 103",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_104": {
            "get": {
                "summary": "Get Resource 104",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 104",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_105": {
            "get": {
                "summary": "Get Resource 105",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 105",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_106": {
            "get": {
                "summary": "Get Resource 106",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 106",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_107": {
            "get": {
                "summary": "Get Resource 107",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 107",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_108": {
            "get": {
                "summary": "Get Resource 108",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 108",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_109": {
            "get": {
                "summary": "Get Resource 109",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 109",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_110": {
            "get": {
                "summary": "Get Resource 110",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 110",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_111": {
            "get": {
                "summary": "Get Resource 111",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 111",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_112": {
            "get": {
                "summary": "Get Resource 112",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 112",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_113": {
            "get": {
                "summary": "Get Resource 113",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 113",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_114": {
            "get": {
                "summary": "Get Resource 114",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 114",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_115": {
            "get": {
                "summary": "Get Resource 115",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 115",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_116": {
            "get": {
                "summary": "Get Resource 116",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 116",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_117": {
            "get": {
                "summary": "Get Resource 117",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 117",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_118": {
            "get": {
                "summary": "Get Resource 118",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 118",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_119": {
            "get": {
                "summary": "Get Resource 119",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 119",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_120": {
            "get": {
                "summary": "Get Resource 120",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 120",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_121": {
            "get": {
                "summary": "Get Resource 121",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 121",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_122": {
            "get": {
                "summary": "Get Resource 122",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 122",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_123": {
            "get": {
                "summary": "Get Resource 123",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 123",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_124": {
            "get": {
                "summary": "Get Resource 124",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 124",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_125": {
            "get": {
                "summary": "Get Resource 125",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 125",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_126": {
            "get": {
                "summary": "Get Resource 126",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 126",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_127": {
            "get": {
                "summary": "Get Resource 127",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 127",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_128": {
            "get": {
                "summary": "Get Resource 128",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 128",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_129": {
            "get": {
                "summary": "Get Resource 129",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 129",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_130": {
            "get": {
                "summary": "Get Resource 130",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 130",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_131": {
            "get": {
                "summary": "Get Resource 131",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 131",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_132": {
            "get": {
                "summary": "Get Resource 132",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 132",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_133": {
            "get": {
                "summary": "Get Resource 133",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 133",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_134": {
            "get": {
                "summary": "Get Resource 134",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 134",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_135": {
            "get": {
                "summary": "Get Resource 135",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 135",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_136": {
            "get": {
                "summary": "Get Resource 136",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 136",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_137": {
            "get": {
                "summary": "Get Resource 137",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 137",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_138": {
            "get": {
                "summary": "Get Resource 138",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 138",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_139": {
            "get": {
                "summary": "Get Resource 139",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 139",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_140": {
            "get": {
                "summary": "Get Resource 140",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 140",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_141": {
            "get": {
                "summary": "Get Resource 141",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 141",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_142": {
            "get": {
                "summary": "Get Resource 142",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 142",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_143": {
            "get": {
                "summary": "Get Resource 143",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 143",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_144": {
            "get": {
                "summary": "Get Resource 144",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 144",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_145": {
            "get": {
                "summary": "Get Resource 145",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 145",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_146": {
            "get": {
                "summary": "Get Resource 146",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 146",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_147": {
            "get": {
                "summary": "Get Resource 147",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 147",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_148": {
            "get": {
                "summary": "Get Resource 148",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 148",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        },
        "/api/v1/resource_149": {
            "get": {
                "summary": "Get Resource 149",
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "id": {
                                            "type": "string"
                                        },
                                        "name": {
                                            "type": "string"
                                        },
                                        "created_at": {
                                            "type": "string",
                                            "format": "date-time"
                                        },
                                        "metadata": {
                                            "type": "object",
                                            "additionalProperties": true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "post": {
                "summary": "Create Resource 149",
                "requestBody": {
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "name": {
                                        "type": "string"
                                    },
                                    "metadata": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                },
                "responses": {
                    "201": {
                        "description": "Created"
                    }
                }
            }
        }
    }
}
"#;
