// @generated automatically by Diesel CLI.

diesel::table! {
    todos (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        title -> Varchar,
        description -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    transactions (transactionid) {
        #[max_length = 255]
        transactionid -> Varchar,
        accountnumber -> Int8,
        #[max_length = 255]
        accountname -> Nullable<Varchar>,
        #[max_length = 255]
        accountcategory -> Nullable<Varchar>,
        #[max_length = 255]
        accountprovider -> Nullable<Varchar>,
        #[max_length = 255]
        bankcode -> Nullable<Varchar>,
        #[max_length = 255]
        password -> Nullable<Varchar>,
        tranamount -> Nullable<Int4>,
        #[max_length = 255]
        trancategory -> Nullable<Varchar>,
        #[max_length = 255]
        channel -> Nullable<Varchar>,
        #[max_length = 255]
        currency -> Nullable<Varchar>,
        paymentdate -> Nullable<Timestamp>,
        #[max_length = 255]
        transignature -> Nullable<Varchar>,
        #[max_length = 255]
        narration -> Nullable<Varchar>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    todos,
    transactions,
);
