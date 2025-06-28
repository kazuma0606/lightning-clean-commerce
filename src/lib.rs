pub mod domain {
    pub mod entity {
        pub mod address;
        pub mod cart;
        pub mod category;
        pub mod common;
        pub mod order;
        pub mod product;
        pub mod product_variant;
        pub mod review;
        pub mod stock_movement;
        pub mod user;
    }
    pub mod repository {
        pub mod address_repository;
        pub mod cart_repository;
        pub mod category_repository;
        pub mod order_repository;
        pub mod product_repository;
        pub mod review_repository;
        pub mod stock_repository;
        pub mod user_repository;
    }
    pub mod utils {
        pub mod error;
        pub mod id_generator;
        pub mod password;
        pub mod validation;
    }
}

pub mod application {
    pub mod dto {
        pub mod auth_dto;
        pub mod user_dto {
            pub mod create_user_request_dto;
            pub mod create_user_response_dto;
            pub mod user_dto;
        }
        pub mod cart_dto;
        pub mod order_dto;
        pub mod product_dto;
        pub mod review_dto;
    }
    pub mod use_cases {
        pub mod auth {
            pub mod login_use_case;
            pub mod logout_use_case;
            pub mod refresh_token_use_case;
            pub mod register_use_case;
        }
        pub mod user {
            pub mod create_user_usecase {
                pub mod create_user_usecase;
                pub mod create_user_usecase_impl;
            }
            pub mod get_user_profile_use_case;
            pub mod manage_address_use_case;
            pub mod update_user_profile_use_case;
        }
        pub mod product {
            pub mod get_product_detail_use_case;
            pub mod get_product_list_use_case;
            pub mod manage_product_use_case;
            pub mod search_product_use_case;
        }
        pub mod cart {
            pub mod add_to_cart_use_case;
            pub mod get_cart_use_case;
            pub mod remove_from_cart_use_case;
            pub mod update_cart_use_case;
        }
        pub mod order {
            pub mod cancel_order_use_case;
            pub mod create_order_use_case;
            pub mod get_order_detail_use_case;
            pub mod get_order_list_use_case;
        }
        pub mod review {
            pub mod create_review_use_case;
            pub mod delete_review_use_case;
            pub mod update_review_use_case;
        }
        pub mod admin {
            pub mod analytics_use_case;
            pub mod order_management_use_case;
            pub mod product_management_use_case;
            pub mod stock_management_use_case;
            pub mod user_management_use_case;
        }
    }
    pub mod service {
        pub mod auth_service;
        pub mod email_service;
        pub mod payment_service;
        pub mod upload_service;
    }
}

pub mod infrastructure {
    pub mod db;
    pub mod repository_impl;
    pub mod database {
        pub mod connection;
        pub mod migration;
        pub mod models {
            pub mod address_model;
            pub mod cart_model;
            pub mod order_model;
            pub mod product_model;
            pub mod review_model;
            pub mod user_model;
        }
    }
    pub mod repository {
        pub mod address_repository_impl;
        pub mod cart_repository_impl;
        pub mod category_repository_impl;
        pub mod order_repository_impl;
        pub mod product_repository_impl;
        pub mod review_repository_impl;
        pub mod stock_repository_impl;
        pub mod user_repository_impl;
    }
    pub mod service {
        pub mod auth_service_impl;
        pub mod email_service_impl;
        pub mod payment_service_impl;
        pub mod upload_service_impl;
    }
    pub mod config {
        pub mod app_config;
        pub mod cors_config;
        pub mod database_config;
    }
}

pub mod adapter {
    pub mod app;

    pub mod controller {
        pub mod admin_controller;
        pub mod auth_controller;
        pub mod cart_controller;
        pub mod category_controller;
        pub mod order_controller;
        pub mod product_controller;
        pub mod review_controller;
        pub mod upload_controller;
        pub mod user_controller;
    }
    pub mod middleware {
        pub mod admin_middleware;
        pub mod auth_middleware;
        pub mod cors_middleware;
        pub mod logging_middleware;
        pub mod rate_limit_middleware;
    }
    pub mod routes {
        pub mod admin_routes;
        pub mod auth_routes;
        pub mod cart_routes;
        pub mod category_routes;
        pub mod order_routes;
        pub mod product_routes;
        pub mod review_routes;
        pub mod upload_routes;
        pub mod user_routes;
    }
}

// runを外部から使えるようにする
pub use adapter::app::run;

pub mod api {
    pub mod response {
        pub mod api_response;
        pub mod error_response;
        pub mod pagination_response;
    }
    pub mod schema {
        pub mod admin_schema;
        pub mod auth_schema;
        pub mod cart_schema;
        pub mod order_schema;
        pub mod product_schema;
        pub mod user_schema;
    }
}

pub mod state {
    pub mod app_state;
}
