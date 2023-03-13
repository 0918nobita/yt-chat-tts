port module Main exposing (main)

import Browser
import Html exposing (Html, button, div, input, text)
import Html.Attributes exposing (class, placeholder, value)
import Html.Events exposing (onClick, onInput)
import Url
import Url.Parser exposing ((</>), s)
import Url.Parser.Query


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
    , message : String
    , url : String
    , urlParseResult : String
    }


init : () -> ( Model, Cmd Msg )
init _ =
    ( { yourName = ""
      , message = ""
      , url = ""
      , urlParseResult = ""
      }
    , Cmd.none
    )


type Msg
    = YourNameChanged String
    | UrlChanged String
    | Send
    | Recv String


parseVideoId : String -> Maybe String
parseVideoId =
    let
        v =
            Url.Parser.Query.string "v"
    in
    Url.fromString
        >> Maybe.andThen (Url.Parser.parse (s "watch" </> Url.Parser.query v))
        >> Maybe.withDefault Nothing


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        YourNameChanged newName ->
            ( { model | yourName = newName }
            , Cmd.none
            )

        UrlChanged newUrl ->
            let
                urlParseResult =
                    if String.isEmpty newUrl then
                        ""

                    else
                        parseVideoId newUrl
                            |> Maybe.map ((++) "Video ID: ")
                            |> Maybe.withDefault "Faild to parse video id from URL"
            in
            ( { model
                | url = newUrl
                , urlParseResult = urlParseResult
              }
            , Cmd.none
            )

        Send ->
            ( model, greet model.yourName )

        Recv message ->
            ( { model | message = message }
            , Cmd.none
            )


subscriptions : Model -> Sub Msg
subscriptions _ =
    messageReceiver Recv


view : Model -> Html Msg
view model =
    div [ class "p-3" ]
        [ div []
            [ input
                [ placeholder "Type your name"
                , value model.yourName
                , onInput YourNameChanged
                , class "rounded bg-transparent text-black border border-gray-400 text-gray-300 px-2 py-1"
                ]
                []
            , button [ onClick Send, class "rounded bg-gray-600 border border-gray-500 text-gray-300 px-2 py-1 ml-2" ] [ text "Greet" ]
            ]
        , div [ class "text-gray-300 py-1" ] [ text model.message ]
        , div []
            [ input
                [ placeholder "Type URL"
                , value model.url
                , onInput UrlChanged
                , class "rounded bg-transparent text-black border border-gray-400 text-gray-300 px-2 py-1"
                ]
                []
            ]
        , div [ class "text-gray-300 py-1" ] [ text model.urlParseResult ]
        ]
