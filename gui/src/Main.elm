port module Main exposing (main)

import Browser
import Html exposing (Html, button, div, input, text)
import Html.Attributes exposing (placeholder, value)
import Html.Events exposing (onClick, onInput)


main : Program () Model Msg
main =
    Browser.element
        { init = init
        , view = view
        , update = update
        , subscriptions = subscriptions
        }


port greet : String -> Cmd msg


port messageReceiver : (String -> msg) -> Sub msg


type alias Model =
    { yourName : String
    , message : Maybe String
    }


init : () -> ( Model, Cmd Msg )
init _ =
    ( { yourName = ""
      , message = Nothing
      }
    , Cmd.none
    )


type Msg
    = YourNameChanged String
    | Send
    | Recv String


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        YourNameChanged newName ->
            ( { model | yourName = newName }
            , Cmd.none
            )

        Send ->
            ( model, greet model.yourName )

        Recv message ->
            ( { model | message = Just message }
            , Cmd.none
            )


subscriptions : Model -> Sub Msg
subscriptions _ =
    messageReceiver Recv


view : Model -> Html Msg
view model =
    div []
        [ div []
            [ input
                [ placeholder "Type your name"
                , value model.yourName
                , onInput YourNameChanged
                ]
                []
            , button [ onClick Send ] [ text "Greet" ]
            ]
        , div [] [ text (Maybe.withDefault "" model.message) ]
        ]
