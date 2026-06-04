use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use std::sync::LazyLock;

use kit_std_ext::HashMapExt;
use kit_std_ext::OsStringExt;
use uhttp::*;

use crate::client::CLIENT_FILES;
use crate::client::CLIENT_FILES_BR;
use crate::client::CLIENT_FILES_GZ;
use crate::ctx::Ctx;

#[deprecated]
#[allow(unused)]
static CONTENT_SECURITY_POLICY: &str = "default-src 'self'; script-src 'self' 'unsafe-inline'; object-src 'none'; base-uri 'none'; form-action 'self'; frame-ancestors 'none';";

pub async fn not_found_any(
  _req: uhttp::Request,
  mut res: uhttp::Response,
  _ctx: Ctx,
) -> uhttp::Result<()> {
  res.header().set("Content-Type", "text/html").await?;
  res.write_all(b"<h1>Not found</h1>").await?;
  res.write_head(StatusCode::NOT_FOUND).await?;
  Ok(())
}

pub async fn not_found_json_any(
  _req: uhttp::Request,
  mut res: uhttp::Response,
  _ctx: Ctx,
) -> uhttp::Result<()> {
  res.header().set("Content-Type", "application/json").await?;
  res.write_all(b"{ \"message\": \"not found\" }").await?;
  res.write_head(StatusCode::NOT_FOUND).await?;
  Ok(())
}

pub fn serve_static_get(
  path: &Path,
  compress: bool,
) -> anyhow::Result<uhttp::router::RouterHandleFunc<Ctx>> {
  let Some(file) = CLIENT_FILES.get_file(path) else {
    return Err(anyhow::anyhow!("File not found {:?}", path));
  };

  let contents = Arc::new(file.contents().to_vec());
  let etag = Arc::new(kit_hash::sha256(&contents));

  let contents_br: Arc<Option<(Vec<u8>, String)>> = {
    if compress {
      let contents = CLIENT_FILES_BR.try_get(path)?.clone();
      let etag = kit_hash::sha256(&contents);
      Arc::new(Some((contents, etag)))
    } else {
      Arc::new(None)
    }
  };
  let contents_gz: Arc<Option<(Vec<u8>, String)>> = {
    if compress {
      let contents = CLIENT_FILES_GZ.try_get(path)?.clone();
      let etag = kit_hash::sha256(&contents);
      Arc::new(Some((contents, etag)))
    } else {
      Arc::new(None)
    }
  };

  // let mut content_security_policy = Arc::new(None::<String>);
  let mut cache_control = Arc::new("max-age=31536000, immutable, public".to_string());
  let mut mime_type = None::<String>;
  if let Some(ext) = path.extension()
    && let Ok(ext) = ext.try_to_string()
    && let Some(found) = MIME_TYPES.get(&ext)
  {
    if ext == "html" {
      // content_security_policy = Arc::new(Some(CONTENT_SECURITY_POLICY.to_string()));
      cache_control = Arc::new("no-cache, must-revalidate".to_string())
    }
    mime_type.replace(found.clone());
  }
  let mime_type = Arc::new(mime_type);

  Ok(Box::new(move |req, mut res, _ctx| {
    let compress = compress;
    let contents = Arc::clone(&contents);
    let contents_br = Arc::clone(&contents_br);
    let contents_gz = Arc::clone(&contents_gz);
    let etag = Arc::clone(&etag);
    // let content_security_policy = Arc::clone(&content_security_policy);
    let cache_control = Arc::clone(&cache_control);
    let mime_type = Arc::clone(&mime_type);

    Box::pin(async move {
      res.header().add("Cache-Control", &cache_control).await?;

      // if let Some(content_security_policy) = content_security_policy.as_ref() {
      //   res
      //     .header()
      //     .add("Content-Security-Policy", content_security_policy)
      //     .await?;
      // }

      if let Some(mime_type) = mime_type.as_ref() {
        res.header().add("Content-Type", mime_type).await?;
      }

      if compress
        && let Some(accept_encoding) = req.headers().get("Accept-Encoding")
        && let Ok(accept_encoding) = accept_encoding.to_str()
      {
        if accept_encoding.contains("br") {
          res.header().add("Content-Encoding", "br").await?;

          let Some((contents, etag)) = contents_br.as_ref() else {
            res.write_all(b"Cache Error").await?;
            res
              .write_head(uhttp::StatusCode::INTERNAL_SERVER_ERROR)
              .await?;
            return Ok(());
          };

          if let Some(if_none_match) = req.headers().get("If-None-Match")
            && if_none_match.to_str()? == etag.as_str()
          {
            res.write_head(uhttp::StatusCode::NOT_MODIFIED).await?;
            return Ok(());
          }

          res.header().add("ETag", etag).await?;
          res.write_all(contents).await?;
          res.write_head(uhttp::StatusCode::OK).await?;
          return Ok(());
        } else if accept_encoding.contains("gzip") {
          res.header().add("Content-Encoding", "gzip").await?;

          let Some((contents, etag)) = contents_gz.as_ref() else {
            res.write_all(b"Cache Error").await?;
            res
              .write_head(uhttp::StatusCode::INTERNAL_SERVER_ERROR)
              .await?;
            return Ok(());
          };

          if let Some(if_none_match) = req.headers().get("If-None-Match")
            && if_none_match.to_str()? == etag.as_str()
          {
            res.write_head(uhttp::StatusCode::NOT_MODIFIED).await?;
            return Ok(());
          }

          res.header().add("ETag", etag).await?;
          res.write_all(contents).await?;
          res.write_head(uhttp::StatusCode::OK).await?;
          return Ok(());
        }
      }

      if let Some(if_none_match) = req.headers().get("If-None-Match")
        && if_none_match.to_str()? == etag.as_str()
      {
        res.write_head(uhttp::StatusCode::NOT_MODIFIED).await?;
        return Ok(());
      }
      res.header().add("ETag", &etag).await?;
      res.write_all(contents.as_slice()).await?;
      res.write_head(uhttp::StatusCode::OK).await?;
      Ok(())
    })
  }))
}

static MIME_TYPES: LazyLock<HashMap<String, String>> = LazyLock::new(|| {
  HashMap::from([
    ("123".to_string(), "application/vnd.lotus-1-2-3".to_string()),
    ("ez".to_string(), "application/andrew-inset".to_string()),
    ("aw".to_string(), "application/applixware".to_string()),
    ("atom".to_string(), "application/atom+xml".to_string()),
    ("atomcat".to_string(), "application/atomcat+xml".to_string()),
    (
      "atomdeleted".to_string(),
      "application/atomdeleted+xml".to_string(),
    ),
    ("atomsvc".to_string(), "application/atomsvc+xml".to_string()),
    ("dwd".to_string(), "application/atsc-dwd+xml".to_string()),
    ("held".to_string(), "application/atsc-held+xml".to_string()),
    ("rsat".to_string(), "application/atsc-rsat+xml".to_string()),
    ("bdoc".to_string(), "application/bdoc".to_string()),
    ("xcs".to_string(), "application/calendar+xml".to_string()),
    ("ccxml".to_string(), "application/ccxml+xml".to_string()),
    ("cdfx".to_string(), "application/cdfx+xml".to_string()),
    (
      "cdmia".to_string(),
      "application/cdmi-capability".to_string(),
    ),
    (
      "cdmic".to_string(),
      "application/cdmi-container".to_string(),
    ),
    ("cdmid".to_string(), "application/cdmi-domain".to_string()),
    ("cdmio".to_string(), "application/cdmi-object".to_string()),
    ("cdmiq".to_string(), "application/cdmi-queue".to_string()),
    ("cpl".to_string(), "application/cpl+xml".to_string()),
    ("cu".to_string(), "application/cu-seeme".to_string()),
    ("mpd".to_string(), "application/dash+xml".to_string()),
    ("mpp".to_string(), "application/dash-patch+xml".to_string()),
    (
      "davmount".to_string(),
      "application/davmount+xml".to_string(),
    ),
    ("dbk".to_string(), "application/docbook+xml".to_string()),
    ("dssc".to_string(), "application/dssc+der".to_string()),
    ("xdssc".to_string(), "application/dssc+xml".to_string()),
    ("es".to_string(), "application/ecmascript".to_string()),
    ("ecma".to_string(), "application/ecmascript".to_string()),
    ("emma".to_string(), "application/emma+xml".to_string()),
    (
      "emotionml".to_string(),
      "application/emotionml+xml".to_string(),
    ),
    ("epub".to_string(), "application/epub+zip".to_string()),
    ("exi".to_string(), "application/exi".to_string()),
    ("exp".to_string(), "application/express".to_string()),
    ("fdt".to_string(), "application/fdt+xml".to_string()),
    ("pfr".to_string(), "application/font-tdpfr".to_string()),
    ("geojson".to_string(), "application/geo+json".to_string()),
    ("gml".to_string(), "application/gml+xml".to_string()),
    ("gpx".to_string(), "application/gpx+xml".to_string()),
    ("gxf".to_string(), "application/gxf".to_string()),
    ("gz".to_string(), "application/gzip".to_string()),
    ("hjson".to_string(), "application/hjson".to_string()),
    ("stk".to_string(), "application/hyperstudio".to_string()),
    ("ink".to_string(), "application/inkml+xml".to_string()),
    ("inkml".to_string(), "application/inkml+xml".to_string()),
    ("ipfix".to_string(), "application/ipfix".to_string()),
    ("its".to_string(), "application/its+xml".to_string()),
    ("jar".to_string(), "application/java-archive".to_string()),
    ("war".to_string(), "application/java-archive".to_string()),
    ("ear".to_string(), "application/java-archive".to_string()),
    (
      "ser".to_string(),
      "application/java-serialized-object".to_string(),
    ),
    ("class".to_string(), "application/java-vm".to_string()),
    ("js".to_string(), "application/javascript".to_string()),
    ("mjs".to_string(), "application/javascript".to_string()),
    ("json".to_string(), "application/json".to_string()),
    ("map".to_string(), "application/json".to_string()),
    ("json5".to_string(), "application/json5".to_string()),
    ("jsonml".to_string(), "application/jsonml+json".to_string()),
    ("jsonld".to_string(), "application/ld+json".to_string()),
    ("lgr".to_string(), "application/lgr+xml".to_string()),
    ("lostxml".to_string(), "application/lost+xml".to_string()),
    ("hqx".to_string(), "application/mac-binhex40".to_string()),
    ("cpt".to_string(), "application/mac-compactpro".to_string()),
    ("mads".to_string(), "application/mads+xml".to_string()),
    (
      "webmanifest".to_string(),
      "application/manifest+json".to_string(),
    ),
    ("mrc".to_string(), "application/marc".to_string()),
    ("mrcx".to_string(), "application/marcxml+xml".to_string()),
    ("ma".to_string(), "application/mathematica".to_string()),
    ("nb".to_string(), "application/mathematica".to_string()),
    ("mb".to_string(), "application/mathematica".to_string()),
    ("mathml".to_string(), "application/mathml+xml".to_string()),
    ("mbox".to_string(), "application/mbox".to_string()),
    (
      "mpf".to_string(),
      "application/media-policy-dataset+xml".to_string(),
    ),
    (
      "mscml".to_string(),
      "application/mediaservercontrol+xml".to_string(),
    ),
    (
      "metalink".to_string(),
      "application/metalink+xml".to_string(),
    ),
    ("meta4".to_string(), "application/metalink4+xml".to_string()),
    ("mets".to_string(), "application/mets+xml".to_string()),
    ("maei".to_string(), "application/mmt-aei+xml".to_string()),
    ("musd".to_string(), "application/mmt-usd+xml".to_string()),
    ("mods".to_string(), "application/mods+xml".to_string()),
    ("m21".to_string(), "application/mp21".to_string()),
    ("mp21".to_string(), "application/mp21".to_string()),
    ("mp4s".to_string(), "application/mp4".to_string()),
    ("m4p".to_string(), "application/mp4".to_string()),
    ("doc".to_string(), "application/msword".to_string()),
    ("dot".to_string(), "application/msword".to_string()),
    ("mxf".to_string(), "application/mxf".to_string()),
    ("nq".to_string(), "application/n-quads".to_string()),
    ("nt".to_string(), "application/n-triples".to_string()),
    ("cjs".to_string(), "application/node".to_string()),
    ("bin".to_string(), "application/octet-stream".to_string()),
    ("dms".to_string(), "application/octet-stream".to_string()),
    ("lrf".to_string(), "application/octet-stream".to_string()),
    ("mar".to_string(), "application/octet-stream".to_string()),
    ("so".to_string(), "application/octet-stream".to_string()),
    ("dist".to_string(), "application/octet-stream".to_string()),
    ("distz".to_string(), "application/octet-stream".to_string()),
    ("pkg".to_string(), "application/octet-stream".to_string()),
    ("bpk".to_string(), "application/octet-stream".to_string()),
    ("dump".to_string(), "application/octet-stream".to_string()),
    ("elc".to_string(), "application/octet-stream".to_string()),
    ("deploy".to_string(), "application/octet-stream".to_string()),
    ("exe".to_string(), "application/x-msdos-program".to_string()),
    ("dll".to_string(), "application/x-msdownload".to_string()),
    (
      "deb".to_string(),
      "application/x-debian-package".to_string(),
    ),
    (
      "dmg".to_string(),
      "application/x-apple-diskimage".to_string(),
    ),
    ("iso".to_string(), "application/x-iso9660-image".to_string()),
    ("img".to_string(), "application/octet-stream".to_string()),
    ("msi".to_string(), "application/x-msdownload".to_string()),
    ("msp".to_string(), "application/octet-stream".to_string()),
    ("msm".to_string(), "application/octet-stream".to_string()),
    ("buffer".to_string(), "application/octet-stream".to_string()),
    ("oda".to_string(), "application/oda".to_string()),
    (
      "opf".to_string(),
      "application/oebps-package+xml".to_string(),
    ),
    ("ogx".to_string(), "application/ogg".to_string()),
    ("omdoc".to_string(), "application/omdoc+xml".to_string()),
    ("onetoc".to_string(), "application/onenote".to_string()),
    ("onetoc2".to_string(), "application/onenote".to_string()),
    ("onetmp".to_string(), "application/onenote".to_string()),
    ("onepkg".to_string(), "application/onenote".to_string()),
    ("oxps".to_string(), "application/oxps".to_string()),
    (
      "relo".to_string(),
      "application/p2p-overlay+xml".to_string(),
    ),
    (
      "xer".to_string(),
      "application/patch-ops-error+xml".to_string(),
    ),
    ("pdf".to_string(), "application/pdf".to_string()),
    ("pgp".to_string(), "application/pgp-encrypted".to_string()),
    ("asc".to_string(), "application/pgp-keys".to_string()),
    ("sig".to_string(), "application/pgp-signature".to_string()),
    ("prf".to_string(), "application/pics-rules".to_string()),
    ("p10".to_string(), "application/pkcs10".to_string()),
    ("p7m".to_string(), "application/pkcs7-mime".to_string()),
    ("p7c".to_string(), "application/pkcs7-mime".to_string()),
    ("p7s".to_string(), "application/pkcs7-signature".to_string()),
    ("p8".to_string(), "application/pkcs8".to_string()),
    ("ac".to_string(), "application/pkix-attr-cert".to_string()),
    ("cer".to_string(), "application/pkix-cert".to_string()),
    ("crl".to_string(), "application/pkix-crl".to_string()),
    (
      "pkipath".to_string(),
      "application/pkix-pkipath".to_string(),
    ),
    ("pki".to_string(), "application/pkixcmp".to_string()),
    ("pls".to_string(), "application/pls+xml".to_string()),
    ("ai".to_string(), "application/postscript".to_string()),
    ("eps".to_string(), "application/postscript".to_string()),
    ("ps".to_string(), "application/postscript".to_string()),
    (
      "provx".to_string(),
      "application/provenance+xml".to_string(),
    ),
    ("cww".to_string(), "application/prs.cww".to_string()),
    ("pskcxml".to_string(), "application/pskc+xml".to_string()),
    ("raml".to_string(), "application/raml+yaml".to_string()),
    ("rdf".to_string(), "application/rdf+xml".to_string()),
    ("owl".to_string(), "application/rdf+xml".to_string()),
    ("rif".to_string(), "application/reginfo+xml".to_string()),
    (
      "rnc".to_string(),
      "application/relax-ng-compact-syntax".to_string(),
    ),
    (
      "rl".to_string(),
      "application/resource-lists+xml".to_string(),
    ),
    (
      "rld".to_string(),
      "application/resource-lists-diff+xml".to_string(),
    ),
    ("rs".to_string(), "application/rls-services+xml".to_string()),
    ("rapd".to_string(), "application/route-apd+xml".to_string()),
    (
      "sls".to_string(),
      "application/route-s-tsid+xml".to_string(),
    ),
    ("rusd".to_string(), "application/route-usd+xml".to_string()),
    (
      "gbr".to_string(),
      "application/rpki-ghostbusters".to_string(),
    ),
    ("mft".to_string(), "application/rpki-manifest".to_string()),
    ("roa".to_string(), "application/rpki-roa".to_string()),
    ("rsd".to_string(), "application/rsd+xml".to_string()),
    ("rss".to_string(), "application/rss+xml".to_string()),
    ("rtf".to_string(), "application/rtf".to_string()),
    ("sbml".to_string(), "application/sbml+xml".to_string()),
    ("scq".to_string(), "application/scvp-cv-request".to_string()),
    (
      "scs".to_string(),
      "application/scvp-cv-response".to_string(),
    ),
    ("spq".to_string(), "application/scvp-vp-request".to_string()),
    (
      "spp".to_string(),
      "application/scvp-vp-response".to_string(),
    ),
    ("sdp".to_string(), "application/sdp".to_string()),
    ("senmlx".to_string(), "application/senml+xml".to_string()),
    ("sensmlx".to_string(), "application/sensml+xml".to_string()),
    (
      "setpay".to_string(),
      "application/set-payment-initiation".to_string(),
    ),
    (
      "setreg".to_string(),
      "application/set-registration-initiation".to_string(),
    ),
    ("shf".to_string(), "application/shf+xml".to_string()),
    ("siv".to_string(), "application/sieve".to_string()),
    ("sieve".to_string(), "application/sieve".to_string()),
    ("smi".to_string(), "application/smil+xml".to_string()),
    ("smil".to_string(), "application/smil+xml".to_string()),
    ("rq".to_string(), "application/sparql-query".to_string()),
    (
      "srx".to_string(),
      "application/sparql-results+xml".to_string(),
    ),
    ("gram".to_string(), "application/srgs".to_string()),
    ("grxml".to_string(), "application/srgs+xml".to_string()),
    ("sru".to_string(), "application/sru+xml".to_string()),
    ("ssdl".to_string(), "application/ssdl+xml".to_string()),
    ("ssml".to_string(), "application/ssml+xml".to_string()),
    ("swidtag".to_string(), "application/swid+xml".to_string()),
    ("tei".to_string(), "application/tei+xml".to_string()),
    ("teicorpus".to_string(), "application/tei+xml".to_string()),
    ("tfi".to_string(), "application/thraud+xml".to_string()),
    (
      "tsd".to_string(),
      "application/timestamped-data".to_string(),
    ),
    ("toml".to_string(), "application/toml".to_string()),
    ("trig".to_string(), "application/trig".to_string()),
    ("ttml".to_string(), "application/ttml+xml".to_string()),
    ("ubj".to_string(), "application/ubjson".to_string()),
    (
      "rsheet".to_string(),
      "application/urc-ressheet+xml".to_string(),
    ),
    (
      "td".to_string(),
      "application/urc-targetdesc+xml".to_string(),
    ),
    (
      "1km".to_string(),
      "application/vnd.1000minds.decision-model+xml".to_string(),
    ),
    (
      "plb".to_string(),
      "application/vnd.3gpp.pic-bw-large".to_string(),
    ),
    (
      "psb".to_string(),
      "application/vnd.3gpp.pic-bw-small".to_string(),
    ),
    (
      "pvb".to_string(),
      "application/vnd.3gpp.pic-bw-var".to_string(),
    ),
    ("tcap".to_string(), "application/vnd.3gpp2.tcap".to_string()),
    (
      "pwn".to_string(),
      "application/vnd.3m.post-it-notes".to_string(),
    ),
    (
      "aso".to_string(),
      "application/vnd.accpac.simply.aso".to_string(),
    ),
    (
      "imp".to_string(),
      "application/vnd.accpac.simply.imp".to_string(),
    ),
    ("acu".to_string(), "application/vnd.acucobol".to_string()),
    ("atc".to_string(), "application/vnd.acucorp".to_string()),
    ("acutc".to_string(), "application/vnd.acucorp".to_string()),
    (
      "air".to_string(),
      "application/vnd.adobe.air-application-installer-package+zip".to_string(),
    ),
    (
      "fcdt".to_string(),
      "application/vnd.adobe.formscentral.fcdt".to_string(),
    ),
    ("fxp".to_string(), "application/vnd.adobe.fxp".to_string()),
    ("fxpl".to_string(), "application/vnd.adobe.fxp".to_string()),
    (
      "xdp".to_string(),
      "application/vnd.adobe.xdp+xml".to_string(),
    ),
    ("xfdf".to_string(), "application/vnd.adobe.xfdf".to_string()),
    ("age".to_string(), "application/vnd.age".to_string()),
    (
      "ahead".to_string(),
      "application/vnd.ahead.space".to_string(),
    ),
    (
      "azf".to_string(),
      "application/vnd.airzip.filesecure.azf".to_string(),
    ),
    (
      "azs".to_string(),
      "application/vnd.airzip.filesecure.azs".to_string(),
    ),
    (
      "azw".to_string(),
      "application/vnd.amazon.ebook".to_string(),
    ),
    (
      "acc".to_string(),
      "application/vnd.americandynamics.acc".to_string(),
    ),
    ("ami".to_string(), "application/vnd.amiga.ami".to_string()),
    (
      "apk".to_string(),
      "application/vnd.android.package-archive".to_string(),
    ),
    (
      "cii".to_string(),
      "application/vnd.anser-web-certificate-issue-initiation".to_string(),
    ),
    (
      "fti".to_string(),
      "application/vnd.anser-web-funds-transfer-initiation".to_string(),
    ),
    (
      "atx".to_string(),
      "application/vnd.antix.game-component".to_string(),
    ),
    (
      "mpkg".to_string(),
      "application/vnd.apple.installer+xml".to_string(),
    ),
    (
      "key".to_string(),
      "application/vnd.apple.keynote".to_string(),
    ),
    (
      "m3u8".to_string(),
      "application/vnd.apple.mpegurl".to_string(),
    ),
    (
      "numbers".to_string(),
      "application/vnd.apple.numbers".to_string(),
    ),
    (
      "pages".to_string(),
      "application/vnd.apple.pages".to_string(),
    ),
    (
      "pkpass".to_string(),
      "application/vnd.apple.pkpass".to_string(),
    ),
    (
      "swi".to_string(),
      "application/vnd.aristanetworks.swi".to_string(),
    ),
    (
      "iota".to_string(),
      "application/vnd.astraea-software.iota".to_string(),
    ),
    ("aep".to_string(), "application/vnd.audiograph".to_string()),
    (
      "bmml".to_string(),
      "application/vnd.balsamiq.bmml+xml".to_string(),
    ),
    (
      "mpm".to_string(),
      "application/vnd.blueice.multipass".to_string(),
    ),
    ("bmi".to_string(), "application/vnd.bmi".to_string()),
    (
      "rep".to_string(),
      "application/vnd.businessobjects".to_string(),
    ),
    (
      "cdxml".to_string(),
      "application/vnd.chemdraw+xml".to_string(),
    ),
    (
      "mmd".to_string(),
      "application/vnd.chipnuts.karaoke-mmd".to_string(),
    ),
    ("cdy".to_string(), "application/vnd.cinderella".to_string()),
    (
      "csl".to_string(),
      "application/vnd.citationstyles.style+xml".to_string(),
    ),
    ("cla".to_string(), "application/vnd.claymore".to_string()),
    ("rp9".to_string(), "application/vnd.cloanto.rp9".to_string()),
    (
      "c4g".to_string(),
      "application/vnd.clonk.c4group".to_string(),
    ),
    (
      "c4d".to_string(),
      "application/vnd.clonk.c4group".to_string(),
    ),
    (
      "c4f".to_string(),
      "application/vnd.clonk.c4group".to_string(),
    ),
    (
      "c4p".to_string(),
      "application/vnd.clonk.c4group".to_string(),
    ),
    (
      "c4u".to_string(),
      "application/vnd.clonk.c4group".to_string(),
    ),
    (
      "c11amc".to_string(),
      "application/vnd.cluetrust.cartomobile-config".to_string(),
    ),
    (
      "c11amz".to_string(),
      "application/vnd.cluetrust.cartomobile-config-pkg".to_string(),
    ),
    ("csp".to_string(), "application/vnd.commonspace".to_string()),
    (
      "cdbcmsg".to_string(),
      "application/vnd.contact.cmsg".to_string(),
    ),
    ("cmc".to_string(), "application/vnd.cosmocaller".to_string()),
    (
      "clkx".to_string(),
      "application/vnd.crick.clicker".to_string(),
    ),
    (
      "clkk".to_string(),
      "application/vnd.crick.clicker.keyboard".to_string(),
    ),
    (
      "clkp".to_string(),
      "application/vnd.crick.clicker.palette".to_string(),
    ),
    (
      "clkt".to_string(),
      "application/vnd.crick.clicker.template".to_string(),
    ),
    (
      "clkw".to_string(),
      "application/vnd.crick.clicker.wordbank".to_string(),
    ),
    (
      "wbs".to_string(),
      "application/vnd.criticaltools.wbs+xml".to_string(),
    ),
    ("pml".to_string(), "application/vnd.ctc-posml".to_string()),
    ("ppd".to_string(), "application/vnd.cups-ppd".to_string()),
    ("car".to_string(), "application/vnd.curl.car".to_string()),
    (
      "pcurl".to_string(),
      "application/vnd.curl.pcurl".to_string(),
    ),
    ("dart".to_string(), "application/vnd.dart".to_string()),
    (
      "rdz".to_string(),
      "application/vnd.data-vision.rdz".to_string(),
    ),
    ("dbf".to_string(), "application/vnd.dbf".to_string()),
    ("uvf".to_string(), "application/vnd.dece.data".to_string()),
    ("uvvf".to_string(), "application/vnd.dece.data".to_string()),
    ("uvd".to_string(), "application/vnd.dece.data".to_string()),
    ("uvvd".to_string(), "application/vnd.dece.data".to_string()),
    (
      "uvt".to_string(),
      "application/vnd.dece.ttml+xml".to_string(),
    ),
    (
      "uvvt".to_string(),
      "application/vnd.dece.ttml+xml".to_string(),
    ),
    (
      "uvx".to_string(),
      "application/vnd.dece.unspecified".to_string(),
    ),
    (
      "uvvx".to_string(),
      "application/vnd.dece.unspecified".to_string(),
    ),
    ("uvz".to_string(), "application/vnd.dece.zip".to_string()),
    ("uvvz".to_string(), "application/vnd.dece.zip".to_string()),
    (
      "fe_launch".to_string(),
      "application/vnd.denovo.fcselayout-link".to_string(),
    ),
    ("dna".to_string(), "application/vnd.dna".to_string()),
    ("mlp".to_string(), "application/vnd.dolby.mlp".to_string()),
    ("dpg".to_string(), "application/vnd.dpgraph".to_string()),
    (
      "dfac".to_string(),
      "application/vnd.dreamfactory".to_string(),
    ),
    (
      "kpxx".to_string(),
      "application/vnd.ds-keypoint".to_string(),
    ),
    ("ait".to_string(), "application/vnd.dvb.ait".to_string()),
    ("svc".to_string(), "application/vnd.dvb.service".to_string()),
    ("geo".to_string(), "application/vnd.dynageo".to_string()),
    (
      "mag".to_string(),
      "application/vnd.ecowin.chart".to_string(),
    ),
    ("nml".to_string(), "application/vnd.enliven".to_string()),
    ("esf".to_string(), "application/vnd.epson.esf".to_string()),
    ("msf".to_string(), "application/vnd.epson.msf".to_string()),
    (
      "qam".to_string(),
      "application/vnd.epson.quickanime".to_string(),
    ),
    ("slt".to_string(), "application/vnd.epson.salt".to_string()),
    ("ssf".to_string(), "application/vnd.epson.ssf".to_string()),
    (
      "es3".to_string(),
      "application/vnd.eszigno3+xml".to_string(),
    ),
    (
      "et3".to_string(),
      "application/vnd.eszigno3+xml".to_string(),
    ),
    ("ez2".to_string(), "application/vnd.ezpix-album".to_string()),
    (
      "ez3".to_string(),
      "application/vnd.ezpix-package".to_string(),
    ),
    ("fdf".to_string(), "application/vnd.fdf".to_string()),
    (
      "mseed".to_string(),
      "application/vnd.fdsn.mseed".to_string(),
    ),
    ("seed".to_string(), "application/vnd.fdsn.seed".to_string()),
    (
      "dataless".to_string(),
      "application/vnd.fdsn.seed".to_string(),
    ),
    ("gph".to_string(), "application/vnd.flographit".to_string()),
    (
      "ftc".to_string(),
      "application/vnd.fluxtime.clip".to_string(),
    ),
    ("fm".to_string(), "application/vnd.framemaker".to_string()),
    (
      "frame".to_string(),
      "application/vnd.framemaker".to_string(),
    ),
    (
      "maker".to_string(),
      "application/vnd.framemaker".to_string(),
    ),
    ("book".to_string(), "application/vnd.framemaker".to_string()),
    ("fnc".to_string(), "application/vnd.frogans.fnc".to_string()),
    ("ltf".to_string(), "application/vnd.frogans.ltf".to_string()),
    (
      "fsc".to_string(),
      "application/vnd.fsc.weblaunch".to_string(),
    ),
    (
      "oas".to_string(),
      "application/vnd.fujitsu.oasys".to_string(),
    ),
    (
      "oa2".to_string(),
      "application/vnd.fujitsu.oasys2".to_string(),
    ),
    (
      "oa3".to_string(),
      "application/vnd.fujitsu.oasys3".to_string(),
    ),
    (
      "fg5".to_string(),
      "application/vnd.fujitsu.oasysgp".to_string(),
    ),
    (
      "bh2".to_string(),
      "application/vnd.fujitsu.oasysprs".to_string(),
    ),
    (
      "ddd".to_string(),
      "application/vnd.fujixerox.ddd".to_string(),
    ),
    (
      "xdw".to_string(),
      "application/vnd.fujixerox.docuworks".to_string(),
    ),
    (
      "xbd".to_string(),
      "application/vnd.fujixerox.docuworks.binder".to_string(),
    ),
    ("fzs".to_string(), "application/vnd.fuzzysheet".to_string()),
    (
      "txd".to_string(),
      "application/vnd.genomatix.tuxedo".to_string(),
    ),
    (
      "ggb".to_string(),
      "application/vnd.geogebra.file".to_string(),
    ),
    (
      "ggt".to_string(),
      "application/vnd.geogebra.tool".to_string(),
    ),
    (
      "gex".to_string(),
      "application/vnd.geometry-explorer".to_string(),
    ),
    (
      "gre".to_string(),
      "application/vnd.geometry-explorer".to_string(),
    ),
    ("gxt".to_string(), "application/vnd.geonext".to_string()),
    ("g2w".to_string(), "application/vnd.geoplan".to_string()),
    ("g3w".to_string(), "application/vnd.geospace".to_string()),
    ("gmx".to_string(), "application/vnd.gmx".to_string()),
    (
      "gdoc".to_string(),
      "application/vnd.google-apps.document".to_string(),
    ),
    (
      "gslides".to_string(),
      "application/vnd.google-apps.presentation".to_string(),
    ),
    (
      "gsheet".to_string(),
      "application/vnd.google-apps.spreadsheet".to_string(),
    ),
    (
      "kml".to_string(),
      "application/vnd.google-earth.kml+xml".to_string(),
    ),
    (
      "kmz".to_string(),
      "application/vnd.google-earth.kmz".to_string(),
    ),
    ("gqf".to_string(), "application/vnd.grafeq".to_string()),
    ("gqs".to_string(), "application/vnd.grafeq".to_string()),
    (
      "gac".to_string(),
      "application/vnd.groove-account".to_string(),
    ),
    ("ghf".to_string(), "application/vnd.groove-help".to_string()),
    (
      "gim".to_string(),
      "application/vnd.groove-identity-message".to_string(),
    ),
    (
      "grv".to_string(),
      "application/vnd.groove-injector".to_string(),
    ),
    (
      "gtm".to_string(),
      "application/vnd.groove-tool-message".to_string(),
    ),
    (
      "tpl".to_string(),
      "application/vnd.groove-tool-template".to_string(),
    ),
    (
      "vcg".to_string(),
      "application/vnd.groove-vcard".to_string(),
    ),
    ("hal".to_string(), "application/vnd.hal+xml".to_string()),
    (
      "zmm".to_string(),
      "application/vnd.handheld-entertainment+xml".to_string(),
    ),
    ("hbci".to_string(), "application/vnd.hbci".to_string()),
    (
      "les".to_string(),
      "application/vnd.hhe.lesson-player".to_string(),
    ),
    ("hpgl".to_string(), "application/vnd.hp-hpgl".to_string()),
    ("hpid".to_string(), "application/vnd.hp-hpid".to_string()),
    ("hps".to_string(), "application/vnd.hp-hps".to_string()),
    ("jlt".to_string(), "application/vnd.hp-jlyt".to_string()),
    ("pcl".to_string(), "application/vnd.hp-pcl".to_string()),
    ("pclxl".to_string(), "application/vnd.hp-pclxl".to_string()),
    (
      "sfd-hdstx".to_string(),
      "application/vnd.hydrostatix.sof-data".to_string(),
    ),
    ("mpy".to_string(), "application/vnd.ibm.minipay".to_string()),
    ("afp".to_string(), "application/vnd.ibm.modcap".to_string()),
    (
      "listafp".to_string(),
      "application/vnd.ibm.modcap".to_string(),
    ),
    (
      "list3820".to_string(),
      "application/vnd.ibm.modcap".to_string(),
    ),
    (
      "irm".to_string(),
      "application/vnd.ibm.rights-management".to_string(),
    ),
    (
      "sc".to_string(),
      "application/vnd.ibm.secure-container".to_string(),
    ),
    ("icc".to_string(), "application/vnd.iccprofile".to_string()),
    ("icm".to_string(), "application/vnd.iccprofile".to_string()),
    ("igl".to_string(), "application/vnd.igloader".to_string()),
    (
      "ivp".to_string(),
      "application/vnd.immervision-ivp".to_string(),
    ),
    (
      "ivu".to_string(),
      "application/vnd.immervision-ivu".to_string(),
    ),
    ("igm".to_string(), "application/vnd.insors.igm".to_string()),
    (
      "xpw".to_string(),
      "application/vnd.intercon.formnet".to_string(),
    ),
    (
      "xpx".to_string(),
      "application/vnd.intercon.formnet".to_string(),
    ),
    ("i2g".to_string(), "application/vnd.intergeo".to_string()),
    ("qbo".to_string(), "application/vnd.intu.qbo".to_string()),
    ("qfx".to_string(), "application/vnd.intu.qfx".to_string()),
    (
      "rcprofile".to_string(),
      "application/vnd.ipunplugged.rcprofile".to_string(),
    ),
    (
      "irp".to_string(),
      "application/vnd.irepository.package+xml".to_string(),
    ),
    ("xpr".to_string(), "application/vnd.is-xpr".to_string()),
    ("fcs".to_string(), "application/vnd.isac.fcs".to_string()),
    ("jam".to_string(), "application/vnd.jam".to_string()),
    (
      "rms".to_string(),
      "application/vnd.jcp.javame.midlet-rms".to_string(),
    ),
    ("jisp".to_string(), "application/vnd.jisp".to_string()),
    (
      "joda".to_string(),
      "application/vnd.joost.joda-archive".to_string(),
    ),
    ("ktz".to_string(), "application/vnd.kahootz".to_string()),
    ("ktr".to_string(), "application/vnd.kahootz".to_string()),
    (
      "karbon".to_string(),
      "application/vnd.kde.karbon".to_string(),
    ),
    ("chrt".to_string(), "application/vnd.kde.kchart".to_string()),
    (
      "kfo".to_string(),
      "application/vnd.kde.kformula".to_string(),
    ),
    ("flw".to_string(), "application/vnd.kde.kivio".to_string()),
    ("kon".to_string(), "application/vnd.kde.kontour".to_string()),
    (
      "kpr".to_string(),
      "application/vnd.kde.kpresenter".to_string(),
    ),
    (
      "kpt".to_string(),
      "application/vnd.kde.kpresenter".to_string(),
    ),
    ("ksp".to_string(), "application/vnd.kde.kspread".to_string()),
    ("kwd".to_string(), "application/vnd.kde.kword".to_string()),
    ("kwt".to_string(), "application/vnd.kde.kword".to_string()),
    ("htke".to_string(), "application/vnd.kenameaapp".to_string()),
    (
      "kia".to_string(),
      "application/vnd.kidspiration".to_string(),
    ),
    ("kne".to_string(), "application/vnd.kinar".to_string()),
    ("knp".to_string(), "application/vnd.kinar".to_string()),
    ("skp".to_string(), "application/vnd.koan".to_string()),
    ("skd".to_string(), "application/vnd.koan".to_string()),
    ("skt".to_string(), "application/vnd.koan".to_string()),
    ("skm".to_string(), "application/vnd.koan".to_string()),
    (
      "sse".to_string(),
      "application/vnd.kodak-descriptor".to_string(),
    ),
    (
      "lasxml".to_string(),
      "application/vnd.las.las+xml".to_string(),
    ),
    (
      "lbd".to_string(),
      "application/vnd.llamagraphics.life-balance.desktop".to_string(),
    ),
    (
      "lbe".to_string(),
      "application/vnd.llamagraphics.life-balance.exchange+xml".to_string(),
    ),
    (
      "apr".to_string(),
      "application/vnd.lotus-approach".to_string(),
    ),
    (
      "pre".to_string(),
      "application/vnd.lotus-freelance".to_string(),
    ),
    ("nsf".to_string(), "application/vnd.lotus-notes".to_string()),
    (
      "org".to_string(),
      "application/vnd.lotus-organizer".to_string(),
    ),
    (
      "scm".to_string(),
      "application/vnd.lotus-screencam".to_string(),
    ),
    (
      "lwp".to_string(),
      "application/vnd.lotus-wordpro".to_string(),
    ),
    (
      "portpkg".to_string(),
      "application/vnd.macports.portpkg".to_string(),
    ),
    (
      "mvt".to_string(),
      "application/vnd.mapbox-vector-tile".to_string(),
    ),
    ("mcd".to_string(), "application/vnd.mcd".to_string()),
    ("mc1".to_string(), "application/vnd.medcalcdata".to_string()),
    (
      "cdkey".to_string(),
      "application/vnd.mediastation.cdkey".to_string(),
    ),
    ("mwf".to_string(), "application/vnd.mfer".to_string()),
    ("mfm".to_string(), "application/vnd.mfmp".to_string()),
    (
      "flo".to_string(),
      "application/vnd.micrografx.flo".to_string(),
    ),
    (
      "igx".to_string(),
      "application/vnd.micrografx.igx".to_string(),
    ),
    ("mif".to_string(), "application/vnd.mif".to_string()),
    ("daf".to_string(), "application/vnd.mobius.daf".to_string()),
    ("dis".to_string(), "application/vnd.mobius.dis".to_string()),
    ("mbk".to_string(), "application/vnd.mobius.mbk".to_string()),
    ("mqy".to_string(), "application/vnd.mobius.mqy".to_string()),
    ("msl".to_string(), "application/vnd.mobius.msl".to_string()),
    ("plc".to_string(), "application/vnd.mobius.plc".to_string()),
    ("txf".to_string(), "application/vnd.mobius.txf".to_string()),
    (
      "mpn".to_string(),
      "application/vnd.mophun.application".to_string(),
    ),
    (
      "mpc".to_string(),
      "application/vnd.mophun.certificate".to_string(),
    ),
    (
      "xul".to_string(),
      "application/vnd.mozilla.xul+xml".to_string(),
    ),
    ("cil".to_string(), "application/vnd.ms-artgalry".to_string()),
    (
      "cab".to_string(),
      "application/vnd.ms-cab-compressed".to_string(),
    ),
    ("xls".to_string(), "application/vnd.ms-excel".to_string()),
    ("xlm".to_string(), "application/vnd.ms-excel".to_string()),
    ("xla".to_string(), "application/vnd.ms-excel".to_string()),
    ("xlc".to_string(), "application/vnd.ms-excel".to_string()),
    ("xlt".to_string(), "application/vnd.ms-excel".to_string()),
    ("xlw".to_string(), "application/vnd.ms-excel".to_string()),
    (
      "xlam".to_string(),
      "application/vnd.ms-excel.addin.macroenabled.12".to_string(),
    ),
    (
      "xlsb".to_string(),
      "application/vnd.ms-excel.sheet.binary.macroenabled.12".to_string(),
    ),
    (
      "xlsm".to_string(),
      "application/vnd.ms-excel.sheet.macroenabled.12".to_string(),
    ),
    (
      "xltm".to_string(),
      "application/vnd.ms-excel.template.macroenabled.12".to_string(),
    ),
    (
      "eot".to_string(),
      "application/vnd.ms-fontobject".to_string(),
    ),
    ("chm".to_string(), "application/vnd.ms-htmlhelp".to_string()),
    ("ims".to_string(), "application/vnd.ms-ims".to_string()),
    ("lrm".to_string(), "application/vnd.ms-lrm".to_string()),
    (
      "thmx".to_string(),
      "application/vnd.ms-officetheme".to_string(),
    ),
    ("msg".to_string(), "application/vnd.ms-outlook".to_string()),
    (
      "cat".to_string(),
      "application/vnd.ms-pki.seccat".to_string(),
    ),
    ("stl".to_string(), "model/stl".to_string()),
    (
      "ppt".to_string(),
      "application/vnd.ms-powerpoint".to_string(),
    ),
    (
      "pps".to_string(),
      "application/vnd.ms-powerpoint".to_string(),
    ),
    (
      "pot".to_string(),
      "application/vnd.ms-powerpoint".to_string(),
    ),
    (
      "ppam".to_string(),
      "application/vnd.ms-powerpoint.addin.macroenabled.12".to_string(),
    ),
    (
      "pptm".to_string(),
      "application/vnd.ms-powerpoint.presentation.macroenabled.12".to_string(),
    ),
    (
      "sldm".to_string(),
      "application/vnd.ms-powerpoint.slide.macroenabled.12".to_string(),
    ),
    (
      "ppsm".to_string(),
      "application/vnd.ms-powerpoint.slideshow.macroenabled.12".to_string(),
    ),
    (
      "potm".to_string(),
      "application/vnd.ms-powerpoint.template.macroenabled.12".to_string(),
    ),
    ("mpt".to_string(), "application/vnd.ms-project".to_string()),
    (
      "docm".to_string(),
      "application/vnd.ms-word.document.macroenabled.12".to_string(),
    ),
    (
      "dotm".to_string(),
      "application/vnd.ms-word.template.macroenabled.12".to_string(),
    ),
    ("wps".to_string(), "application/vnd.ms-works".to_string()),
    ("wks".to_string(), "application/vnd.ms-works".to_string()),
    ("wcm".to_string(), "application/vnd.ms-works".to_string()),
    ("wdb".to_string(), "application/vnd.ms-works".to_string()),
    ("wpl".to_string(), "application/vnd.ms-wpl".to_string()),
    (
      "xps".to_string(),
      "application/vnd.ms-xpsdocument".to_string(),
    ),
    ("mseq".to_string(), "application/vnd.mseq".to_string()),
    ("mus".to_string(), "application/vnd.musician".to_string()),
    (
      "msty".to_string(),
      "application/vnd.muvee.style".to_string(),
    ),
    ("taglet".to_string(), "application/vnd.mynfc".to_string()),
    (
      "nlu".to_string(),
      "application/vnd.neurolanguage.nlu".to_string(),
    ),
    ("ntf".to_string(), "application/vnd.nitf".to_string()),
    ("nitf".to_string(), "application/vnd.nitf".to_string()),
    (
      "nnd".to_string(),
      "application/vnd.noblenet-directory".to_string(),
    ),
    (
      "nns".to_string(),
      "application/vnd.noblenet-sealer".to_string(),
    ),
    (
      "nnw".to_string(),
      "application/vnd.noblenet-web".to_string(),
    ),
    (
      "ngdat".to_string(),
      "application/vnd.nokia.n-gage.data".to_string(),
    ),
    (
      "n-gage".to_string(),
      "application/vnd.nokia.n-gage.symbian.install".to_string(),
    ),
    (
      "rpst".to_string(),
      "application/vnd.nokia.radio-preset".to_string(),
    ),
    (
      "rpss".to_string(),
      "application/vnd.nokia.radio-presets".to_string(),
    ),
    (
      "edm".to_string(),
      "application/vnd.novadigm.edm".to_string(),
    ),
    (
      "edx".to_string(),
      "application/vnd.novadigm.edx".to_string(),
    ),
    (
      "ext".to_string(),
      "application/vnd.novadigm.ext".to_string(),
    ),
    (
      "odc".to_string(),
      "application/vnd.oasis.opendocument.chart".to_string(),
    ),
    (
      "otc".to_string(),
      "application/vnd.oasis.opendocument.chart-template".to_string(),
    ),
    (
      "odb".to_string(),
      "application/vnd.oasis.opendocument.database".to_string(),
    ),
    (
      "odf".to_string(),
      "application/vnd.oasis.opendocument.formula".to_string(),
    ),
    (
      "odft".to_string(),
      "application/vnd.oasis.opendocument.formula-template".to_string(),
    ),
    (
      "odg".to_string(),
      "application/vnd.oasis.opendocument.graphics".to_string(),
    ),
    (
      "otg".to_string(),
      "application/vnd.oasis.opendocument.graphics-template".to_string(),
    ),
    (
      "odi".to_string(),
      "application/vnd.oasis.opendocument.image".to_string(),
    ),
    (
      "oti".to_string(),
      "application/vnd.oasis.opendocument.image-template".to_string(),
    ),
    (
      "odp".to_string(),
      "application/vnd.oasis.opendocument.presentation".to_string(),
    ),
    (
      "otp".to_string(),
      "application/vnd.oasis.opendocument.presentation-template".to_string(),
    ),
    (
      "ods".to_string(),
      "application/vnd.oasis.opendocument.spreadsheet".to_string(),
    ),
    (
      "ots".to_string(),
      "application/vnd.oasis.opendocument.spreadsheet-template".to_string(),
    ),
    (
      "odt".to_string(),
      "application/vnd.oasis.opendocument.text".to_string(),
    ),
    (
      "odm".to_string(),
      "application/vnd.oasis.opendocument.text-master".to_string(),
    ),
    (
      "ott".to_string(),
      "application/vnd.oasis.opendocument.text-template".to_string(),
    ),
    (
      "oth".to_string(),
      "application/vnd.oasis.opendocument.text-web".to_string(),
    ),
    ("xo".to_string(), "application/vnd.olpc-sugar".to_string()),
    ("dd2".to_string(), "application/vnd.oma.dd2+xml".to_string()),
    (
      "obgx".to_string(),
      "application/vnd.openblox.game+xml".to_string(),
    ),
    (
      "oxt".to_string(),
      "application/vnd.openofficeorg.extension".to_string(),
    ),
    (
      "osm".to_string(),
      "application/vnd.openstreetmap.data+xml".to_string(),
    ),
    (
      "pptx".to_string(),
      "application/vnd.openxmlformats-officedocument.presentationml.presentation".to_string(),
    ),
    (
      "sldx".to_string(),
      "application/vnd.openxmlformats-officedocument.presentationml.slide".to_string(),
    ),
    (
      "ppsx".to_string(),
      "application/vnd.openxmlformats-officedocument.presentationml.slideshow".to_string(),
    ),
    (
      "potx".to_string(),
      "application/vnd.openxmlformats-officedocument.presentationml.template".to_string(),
    ),
    (
      "xlsx".to_string(),
      "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet".to_string(),
    ),
    (
      "xltx".to_string(),
      "application/vnd.openxmlformats-officedocument.spreadsheetml.template".to_string(),
    ),
    (
      "docx".to_string(),
      "application/vnd.openxmlformats-officedocument.wordprocessingml.document".to_string(),
    ),
    (
      "dotx".to_string(),
      "application/vnd.openxmlformats-officedocument.wordprocessingml.template".to_string(),
    ),
    (
      "mgp".to_string(),
      "application/vnd.osgeo.mapguide.package".to_string(),
    ),
    ("dp".to_string(), "application/vnd.osgi.dp".to_string()),
    (
      "esa".to_string(),
      "application/vnd.osgi.subsystem".to_string(),
    ),
    ("pdb".to_string(), "application/vnd.palm".to_string()),
    ("pqa".to_string(), "application/vnd.palm".to_string()),
    ("oprc".to_string(), "application/vnd.palm".to_string()),
    ("paw".to_string(), "application/vnd.pawaafile".to_string()),
    ("str".to_string(), "application/vnd.pg.format".to_string()),
    ("ei6".to_string(), "application/vnd.pg.osasli".to_string()),
    ("efif".to_string(), "application/vnd.picsel".to_string()),
    ("wg".to_string(), "application/vnd.pmi.widget".to_string()),
    ("plf".to_string(), "application/vnd.pocketlearn".to_string()),
    (
      "pbd".to_string(),
      "application/vnd.powerbuilder6".to_string(),
    ),
    (
      "box".to_string(),
      "application/vnd.previewsystems.box".to_string(),
    ),
    (
      "mgz".to_string(),
      "application/vnd.proteus.magazine".to_string(),
    ),
    (
      "qps".to_string(),
      "application/vnd.publishare-delta-tree".to_string(),
    ),
    ("ptid".to_string(), "application/vnd.pvi.ptid1".to_string()),
    (
      "qxd".to_string(),
      "application/vnd.quark.quarkxpress".to_string(),
    ),
    (
      "qxt".to_string(),
      "application/vnd.quark.quarkxpress".to_string(),
    ),
    (
      "qwd".to_string(),
      "application/vnd.quark.quarkxpress".to_string(),
    ),
    (
      "qwt".to_string(),
      "application/vnd.quark.quarkxpress".to_string(),
    ),
    (
      "qxl".to_string(),
      "application/vnd.quark.quarkxpress".to_string(),
    ),
    (
      "qxb".to_string(),
      "application/vnd.quark.quarkxpress".to_string(),
    ),
    ("rar".to_string(), "application/vnd.rar".to_string()),
    ("bed".to_string(), "application/vnd.realvnc.bed".to_string()),
    (
      "mxl".to_string(),
      "application/vnd.recordare.musicxml".to_string(),
    ),
    (
      "musicxml".to_string(),
      "application/vnd.recordare.musicxml+xml".to_string(),
    ),
    (
      "cryptonote".to_string(),
      "application/vnd.rig.cryptonote".to_string(),
    ),
    ("cod".to_string(), "application/vnd.rim.cod".to_string()),
    ("rm".to_string(), "application/vnd.rn-realmedia".to_string()),
    (
      "rmvb".to_string(),
      "application/vnd.rn-realmedia-vbr".to_string(),
    ),
    (
      "link66".to_string(),
      "application/vnd.route66.link66+xml".to_string(),
    ),
    (
      "st".to_string(),
      "application/vnd.sailingtracker.track".to_string(),
    ),
    ("see".to_string(), "application/vnd.seemail".to_string()),
    ("sema".to_string(), "application/vnd.sema".to_string()),
    ("semd".to_string(), "application/vnd.semd".to_string()),
    ("semf".to_string(), "application/vnd.semf".to_string()),
    (
      "ifm".to_string(),
      "application/vnd.shana.informed.formdata".to_string(),
    ),
    (
      "itp".to_string(),
      "application/vnd.shana.informed.formtemplate".to_string(),
    ),
    (
      "iif".to_string(),
      "application/vnd.shana.informed.interchange".to_string(),
    ),
    (
      "ipk".to_string(),
      "application/vnd.shana.informed.package".to_string(),
    ),
    (
      "twd".to_string(),
      "application/vnd.simtech-mindmapper".to_string(),
    ),
    (
      "twds".to_string(),
      "application/vnd.simtech-mindmapper".to_string(),
    ),
    ("mmf".to_string(), "application/vnd.smaf".to_string()),
    (
      "teacher".to_string(),
      "application/vnd.smart.teacher".to_string(),
    ),
    (
      "fo".to_string(),
      "application/vnd.software602.filler.form+xml".to_string(),
    ),
    (
      "sdkm".to_string(),
      "application/vnd.solent.sdkm+xml".to_string(),
    ),
    (
      "sdkd".to_string(),
      "application/vnd.solent.sdkm+xml".to_string(),
    ),
    (
      "dxp".to_string(),
      "application/vnd.spotfire.dxp".to_string(),
    ),
    (
      "sfs".to_string(),
      "application/vnd.spotfire.sfs".to_string(),
    ),
    (
      "sdc".to_string(),
      "application/vnd.stardivision.calc".to_string(),
    ),
    (
      "sda".to_string(),
      "application/vnd.stardivision.draw".to_string(),
    ),
    (
      "sdd".to_string(),
      "application/vnd.stardivision.impress".to_string(),
    ),
    (
      "smf".to_string(),
      "application/vnd.stardivision.math".to_string(),
    ),
    (
      "sdw".to_string(),
      "application/vnd.stardivision.writer".to_string(),
    ),
    (
      "vor".to_string(),
      "application/vnd.stardivision.writer".to_string(),
    ),
    (
      "sgl".to_string(),
      "application/vnd.stardivision.writer-global".to_string(),
    ),
    (
      "smzip".to_string(),
      "application/vnd.stepmania.package".to_string(),
    ),
    (
      "sm".to_string(),
      "application/vnd.stepmania.stepchart".to_string(),
    ),
    (
      "wadl".to_string(),
      "application/vnd.sun.wadl+xml".to_string(),
    ),
    (
      "sxc".to_string(),
      "application/vnd.sun.xml.calc".to_string(),
    ),
    (
      "stc".to_string(),
      "application/vnd.sun.xml.calc.template".to_string(),
    ),
    (
      "sxd".to_string(),
      "application/vnd.sun.xml.draw".to_string(),
    ),
    (
      "std".to_string(),
      "application/vnd.sun.xml.draw.template".to_string(),
    ),
    (
      "sxi".to_string(),
      "application/vnd.sun.xml.impress".to_string(),
    ),
    (
      "sti".to_string(),
      "application/vnd.sun.xml.impress.template".to_string(),
    ),
    (
      "sxm".to_string(),
      "application/vnd.sun.xml.math".to_string(),
    ),
    (
      "sxw".to_string(),
      "application/vnd.sun.xml.writer".to_string(),
    ),
    (
      "sxg".to_string(),
      "application/vnd.sun.xml.writer.global".to_string(),
    ),
    (
      "stw".to_string(),
      "application/vnd.sun.xml.writer.template".to_string(),
    ),
    (
      "sus".to_string(),
      "application/vnd.sus-calendar".to_string(),
    ),
    (
      "susp".to_string(),
      "application/vnd.sus-calendar".to_string(),
    ),
    ("svd".to_string(), "application/vnd.svd".to_string()),
    (
      "sis".to_string(),
      "application/vnd.symbian.install".to_string(),
    ),
    (
      "sisx".to_string(),
      "application/vnd.symbian.install".to_string(),
    ),
    ("xsm".to_string(), "application/vnd.syncml+xml".to_string()),
    (
      "bdm".to_string(),
      "application/vnd.syncml.dm+wbxml".to_string(),
    ),
    (
      "xdm".to_string(),
      "application/vnd.syncml.dm+xml".to_string(),
    ),
    (
      "ddf".to_string(),
      "application/vnd.syncml.dmddf+xml".to_string(),
    ),
    (
      "tao".to_string(),
      "application/vnd.tao.intent-module-archive".to_string(),
    ),
    (
      "pcap".to_string(),
      "application/vnd.tcpdump.pcap".to_string(),
    ),
    (
      "cap".to_string(),
      "application/vnd.tcpdump.pcap".to_string(),
    ),
    (
      "dmp".to_string(),
      "application/vnd.tcpdump.pcap".to_string(),
    ),
    (
      "tmo".to_string(),
      "application/vnd.tmobile-livetv".to_string(),
    ),
    ("tpt".to_string(), "application/vnd.trid.tpt".to_string()),
    (
      "mxs".to_string(),
      "application/vnd.triscape.mxs".to_string(),
    ),
    ("tra".to_string(), "application/vnd.trueapp".to_string()),
    ("ufd".to_string(), "application/vnd.ufdl".to_string()),
    ("ufdl".to_string(), "application/vnd.ufdl".to_string()),
    ("utz".to_string(), "application/vnd.uiq.theme".to_string()),
    ("umj".to_string(), "application/vnd.umajin".to_string()),
    ("unityweb".to_string(), "application/vnd.unity".to_string()),
    ("uoml".to_string(), "application/vnd.uoml+xml".to_string()),
    ("vcx".to_string(), "application/vnd.vcx".to_string()),
    ("vsd".to_string(), "application/vnd.visio".to_string()),
    ("vst".to_string(), "application/vnd.visio".to_string()),
    ("vss".to_string(), "application/vnd.visio".to_string()),
    ("vsw".to_string(), "application/vnd.visio".to_string()),
    ("vis".to_string(), "application/vnd.visionary".to_string()),
    ("vsf".to_string(), "application/vnd.vsf".to_string()),
    ("wbxml".to_string(), "application/vnd.wap.wbxml".to_string()),
    ("wmlc".to_string(), "application/vnd.wap.wmlc".to_string()),
    (
      "wmlsc".to_string(),
      "application/vnd.wap.wmlscriptc".to_string(),
    ),
    ("wtb".to_string(), "application/vnd.webturbo".to_string()),
    (
      "nbp".to_string(),
      "application/vnd.wolfram.player".to_string(),
    ),
    ("wpd".to_string(), "application/vnd.wordperfect".to_string()),
    ("wqd".to_string(), "application/vnd.wqd".to_string()),
    ("stf".to_string(), "application/vnd.wt.stf".to_string()),
    ("xar".to_string(), "application/vnd.xara".to_string()),
    ("xfdl".to_string(), "application/vnd.xfdl".to_string()),
    (
      "hvd".to_string(),
      "application/vnd.yamaha.hv-dic".to_string(),
    ),
    (
      "hvs".to_string(),
      "application/vnd.yamaha.hv-script".to_string(),
    ),
    (
      "hvp".to_string(),
      "application/vnd.yamaha.hv-voice".to_string(),
    ),
    (
      "osf".to_string(),
      "application/vnd.yamaha.openscoreformat".to_string(),
    ),
    (
      "osfpvg".to_string(),
      "application/vnd.yamaha.openscoreformat.osfpvg+xml".to_string(),
    ),
    (
      "saf".to_string(),
      "application/vnd.yamaha.smaf-audio".to_string(),
    ),
    (
      "spf".to_string(),
      "application/vnd.yamaha.smaf-phrase".to_string(),
    ),
    (
      "cmp".to_string(),
      "application/vnd.yellowriver-custom-menu".to_string(),
    ),
    ("zir".to_string(), "application/vnd.zul".to_string()),
    ("zirz".to_string(), "application/vnd.zul".to_string()),
    (
      "zaz".to_string(),
      "application/vnd.zzazz.deck+xml".to_string(),
    ),
    ("vxml".to_string(), "application/voicexml+xml".to_string()),
    ("wasm".to_string(), "application/wasm".to_string()),
    ("wif".to_string(), "application/watcherinfo+xml".to_string()),
    ("wgt".to_string(), "application/widget".to_string()),
    ("hlp".to_string(), "application/winhlp".to_string()),
    ("wsdl".to_string(), "application/wsdl+xml".to_string()),
    (
      "wspolicy".to_string(),
      "application/wspolicy+xml".to_string(),
    ),
    ("7z".to_string(), "application/x-7z-compressed".to_string()),
    ("abw".to_string(), "application/x-abiword".to_string()),
    (
      "ace".to_string(),
      "application/x-ace-compressed".to_string(),
    ),
    ("arj".to_string(), "application/x-arj".to_string()),
    (
      "aab".to_string(),
      "application/x-authorware-bin".to_string(),
    ),
    (
      "x32".to_string(),
      "application/x-authorware-bin".to_string(),
    ),
    (
      "u32".to_string(),
      "application/x-authorware-bin".to_string(),
    ),
    (
      "vox".to_string(),
      "application/x-authorware-bin".to_string(),
    ),
    (
      "aam".to_string(),
      "application/x-authorware-map".to_string(),
    ),
    (
      "aas".to_string(),
      "application/x-authorware-seg".to_string(),
    ),
    ("bcpio".to_string(), "application/x-bcpio".to_string()),
    (
      "torrent".to_string(),
      "application/x-bittorrent".to_string(),
    ),
    ("blb".to_string(), "application/x-blorb".to_string()),
    ("blorb".to_string(), "application/x-blorb".to_string()),
    ("bz".to_string(), "application/x-bzip".to_string()),
    ("bz2".to_string(), "application/x-bzip2".to_string()),
    ("boz".to_string(), "application/x-bzip2".to_string()),
    ("cbr".to_string(), "application/x-cbr".to_string()),
    ("cba".to_string(), "application/x-cbr".to_string()),
    ("cbt".to_string(), "application/x-cbr".to_string()),
    ("cbz".to_string(), "application/x-cbr".to_string()),
    ("cb7".to_string(), "application/x-cbr".to_string()),
    ("vcd".to_string(), "application/x-cdlink".to_string()),
    (
      "cfs".to_string(),
      "application/x-cfs-compressed".to_string(),
    ),
    ("chat".to_string(), "application/x-chat".to_string()),
    ("pgn".to_string(), "application/x-chess-pgn".to_string()),
    (
      "crx".to_string(),
      "application/x-chrome-extension".to_string(),
    ),
    ("cco".to_string(), "application/x-cocoa".to_string()),
    ("nsc".to_string(), "application/x-conference".to_string()),
    ("cpio".to_string(), "application/x-cpio".to_string()),
    ("csh".to_string(), "application/x-csh".to_string()),
    (
      "udeb".to_string(),
      "application/x-debian-package".to_string(),
    ),
    (
      "dgc".to_string(),
      "application/x-dgc-compressed".to_string(),
    ),
    ("dir".to_string(), "application/x-director".to_string()),
    ("dcr".to_string(), "application/x-director".to_string()),
    ("dxr".to_string(), "application/x-director".to_string()),
    ("cst".to_string(), "application/x-director".to_string()),
    ("cct".to_string(), "application/x-director".to_string()),
    ("cxt".to_string(), "application/x-director".to_string()),
    ("w3d".to_string(), "application/x-director".to_string()),
    ("fgd".to_string(), "application/x-director".to_string()),
    ("swa".to_string(), "application/x-director".to_string()),
    ("wad".to_string(), "application/x-doom".to_string()),
    ("ncx".to_string(), "application/x-dtbncx+xml".to_string()),
    ("dtb".to_string(), "application/x-dtbook+xml".to_string()),
    (
      "res".to_string(),
      "application/x-dtbresource+xml".to_string(),
    ),
    ("dvi".to_string(), "application/x-dvi".to_string()),
    ("evy".to_string(), "application/x-envoy".to_string()),
    ("eva".to_string(), "application/x-eva".to_string()),
    ("bdf".to_string(), "application/x-font-bdf".to_string()),
    (
      "gsf".to_string(),
      "application/x-font-ghostscript".to_string(),
    ),
    (
      "psf".to_string(),
      "application/x-font-linux-psf".to_string(),
    ),
    ("pcf".to_string(), "application/x-font-pcf".to_string()),
    ("snf".to_string(), "application/x-font-snf".to_string()),
    ("pfa".to_string(), "application/x-font-type1".to_string()),
    ("pfb".to_string(), "application/x-font-type1".to_string()),
    ("pfm".to_string(), "application/x-font-type1".to_string()),
    ("afm".to_string(), "application/x-font-type1".to_string()),
    ("arc".to_string(), "application/x-freearc".to_string()),
    ("spl".to_string(), "application/x-futuresplash".to_string()),
    (
      "gca".to_string(),
      "application/x-gca-compressed".to_string(),
    ),
    ("ulx".to_string(), "application/x-glulx".to_string()),
    ("gnumeric".to_string(), "application/x-gnumeric".to_string()),
    ("gramps".to_string(), "application/x-gramps-xml".to_string()),
    ("gtar".to_string(), "application/x-gtar".to_string()),
    ("hdf".to_string(), "application/x-hdf".to_string()),
    ("php".to_string(), "application/x-httpd-php".to_string()),
    (
      "install".to_string(),
      "application/x-install-instructions".to_string(),
    ),
    (
      "jardiff".to_string(),
      "application/x-java-archive-diff".to_string(),
    ),
    (
      "jnlp".to_string(),
      "application/x-java-jnlp-file".to_string(),
    ),
    ("kdbx".to_string(), "application/x-keepass2".to_string()),
    ("latex".to_string(), "application/x-latex".to_string()),
    ("luac".to_string(), "application/x-lua-bytecode".to_string()),
    (
      "lzh".to_string(),
      "application/x-lzh-compressed".to_string(),
    ),
    (
      "lha".to_string(),
      "application/x-lzh-compressed".to_string(),
    ),
    ("run".to_string(), "application/x-makeself".to_string()),
    ("mie".to_string(), "application/x-mie".to_string()),
    (
      "prc".to_string(),
      "application/x-mobipocket-ebook".to_string(),
    ),
    (
      "mobi".to_string(),
      "application/x-mobipocket-ebook".to_string(),
    ),
    (
      "application".to_string(),
      "application/x-ms-application".to_string(),
    ),
    ("lnk".to_string(), "application/x-ms-shortcut".to_string()),
    ("wmd".to_string(), "application/x-ms-wmd".to_string()),
    ("wmz".to_string(), "application/x-ms-wmz".to_string()),
    ("xbap".to_string(), "application/x-ms-xbap".to_string()),
    ("mdb".to_string(), "application/x-msaccess".to_string()),
    ("obd".to_string(), "application/x-msbinder".to_string()),
    ("crd".to_string(), "application/x-mscardfile".to_string()),
    ("clp".to_string(), "application/x-msclip".to_string()),
    ("com".to_string(), "application/x-msdownload".to_string()),
    ("bat".to_string(), "application/x-msdownload".to_string()),
    ("mvb".to_string(), "application/x-msmediaview".to_string()),
    ("m13".to_string(), "application/x-msmediaview".to_string()),
    ("m14".to_string(), "application/x-msmediaview".to_string()),
    ("wmf".to_string(), "image/wmf".to_string()),
    ("emf".to_string(), "image/emf".to_string()),
    ("emz".to_string(), "application/x-msmetafile".to_string()),
    ("mny".to_string(), "application/x-msmoney".to_string()),
    ("pub".to_string(), "application/x-mspublisher".to_string()),
    ("scd".to_string(), "application/x-msschedule".to_string()),
    ("trm".to_string(), "application/x-msterminal".to_string()),
    ("wri".to_string(), "application/x-mswrite".to_string()),
    ("nc".to_string(), "application/x-netcdf".to_string()),
    ("cdf".to_string(), "application/x-netcdf".to_string()),
    (
      "pac".to_string(),
      "application/x-ns-proxy-autoconfig".to_string(),
    ),
    ("nzb".to_string(), "application/x-nzb".to_string()),
    ("pl".to_string(), "application/x-perl".to_string()),
    ("pm".to_string(), "application/x-perl".to_string()),
    ("p12".to_string(), "application/x-pkcs12".to_string()),
    ("pfx".to_string(), "application/x-pkcs12".to_string()),
    (
      "p7b".to_string(),
      "application/x-pkcs7-certificates".to_string(),
    ),
    (
      "spc".to_string(),
      "application/x-pkcs7-certificates".to_string(),
    ),
    (
      "p7r".to_string(),
      "application/x-pkcs7-certreqresp".to_string(),
    ),
    (
      "rpm".to_string(),
      "application/x-redhat-package-manager".to_string(),
    ),
    (
      "ris".to_string(),
      "application/x-research-info-systems".to_string(),
    ),
    ("sea".to_string(), "application/x-sea".to_string()),
    ("sh".to_string(), "application/x-sh".to_string()),
    ("shar".to_string(), "application/x-shar".to_string()),
    (
      "swf".to_string(),
      "application/x-shockwave-flash".to_string(),
    ),
    (
      "xap".to_string(),
      "application/x-silverlight-app".to_string(),
    ),
    ("sql".to_string(), "application/x-sql".to_string()),
    ("sit".to_string(), "application/x-stuffit".to_string()),
    ("sitx".to_string(), "application/x-stuffitx".to_string()),
    ("srt".to_string(), "application/x-subrip".to_string()),
    ("sv4cpio".to_string(), "application/x-sv4cpio".to_string()),
    ("sv4crc".to_string(), "application/x-sv4crc".to_string()),
    ("t3".to_string(), "application/x-t3vm-image".to_string()),
    ("gam".to_string(), "application/x-tads".to_string()),
    ("tar".to_string(), "application/x-tar".to_string()),
    ("tcl".to_string(), "application/x-tcl".to_string()),
    ("tk".to_string(), "application/x-tcl".to_string()),
    ("tex".to_string(), "application/x-tex".to_string()),
    ("tfm".to_string(), "application/x-tex-tfm".to_string()),
    ("texinfo".to_string(), "application/x-texinfo".to_string()),
    ("texi".to_string(), "application/x-texinfo".to_string()),
    ("obj".to_string(), "model/obj".to_string()),
    ("ustar".to_string(), "application/x-ustar".to_string()),
    (
      "hdd".to_string(),
      "application/x-virtualbox-hdd".to_string(),
    ),
    (
      "ova".to_string(),
      "application/x-virtualbox-ova".to_string(),
    ),
    (
      "ovf".to_string(),
      "application/x-virtualbox-ovf".to_string(),
    ),
    (
      "vbox".to_string(),
      "application/x-virtualbox-vbox".to_string(),
    ),
    (
      "vbox-extpack".to_string(),
      "application/x-virtualbox-vbox-extpack".to_string(),
    ),
    (
      "vdi".to_string(),
      "application/x-virtualbox-vdi".to_string(),
    ),
    (
      "vhd".to_string(),
      "application/x-virtualbox-vhd".to_string(),
    ),
    (
      "vmdk".to_string(),
      "application/x-virtualbox-vmdk".to_string(),
    ),
    ("src".to_string(), "application/x-wais-source".to_string()),
    (
      "webapp".to_string(),
      "application/x-web-app-manifest+json".to_string(),
    ),
    ("der".to_string(), "application/x-x509-ca-cert".to_string()),
    ("crt".to_string(), "application/x-x509-ca-cert".to_string()),
    ("pem".to_string(), "application/x-x509-ca-cert".to_string()),
    ("fig".to_string(), "application/x-xfig".to_string()),
    ("xlf".to_string(), "application/xliff+xml".to_string()),
    ("xpi".to_string(), "application/x-xpinstall".to_string()),
    ("xz".to_string(), "application/x-xz".to_string()),
    ("z1".to_string(), "application/x-zmachine".to_string()),
    ("z2".to_string(), "application/x-zmachine".to_string()),
    ("z3".to_string(), "application/x-zmachine".to_string()),
    ("z4".to_string(), "application/x-zmachine".to_string()),
    ("z5".to_string(), "application/x-zmachine".to_string()),
    ("z6".to_string(), "application/x-zmachine".to_string()),
    ("z7".to_string(), "application/x-zmachine".to_string()),
    ("z8".to_string(), "application/x-zmachine".to_string()),
    ("xaml".to_string(), "application/xaml+xml".to_string()),
    ("xav".to_string(), "application/xcap-att+xml".to_string()),
    ("xca".to_string(), "application/xcap-caps+xml".to_string()),
    ("xdf".to_string(), "application/xcap-diff+xml".to_string()),
    ("xel".to_string(), "application/xcap-el+xml".to_string()),
    ("xns".to_string(), "application/xcap-ns+xml".to_string()),
    ("xenc".to_string(), "application/xenc+xml".to_string()),
    ("xhtml".to_string(), "application/xhtml+xml".to_string()),
    ("xht".to_string(), "application/xhtml+xml".to_string()),
    ("xml".to_string(), "application/xml".to_string()),
    ("xsl".to_string(), "application/xml".to_string()),
    ("xsd".to_string(), "application/xml".to_string()),
    ("rng".to_string(), "application/xml".to_string()),
    ("dtd".to_string(), "application/xml-dtd".to_string()),
    ("xop".to_string(), "application/xop+xml".to_string()),
    ("xpl".to_string(), "application/xproc+xml".to_string()),
    ("xslt".to_string(), "application/xslt+xml".to_string()),
    ("xspf".to_string(), "application/xspf+xml".to_string()),
    ("mxml".to_string(), "application/xv+xml".to_string()),
    ("xhvml".to_string(), "application/xv+xml".to_string()),
    ("xvml".to_string(), "application/xv+xml".to_string()),
    ("xvm".to_string(), "application/xv+xml".to_string()),
    ("yang".to_string(), "application/yang".to_string()),
    ("yin".to_string(), "application/yin+xml".to_string()),
    ("zip".to_string(), "application/zip".to_string()),
    ("3gpp".to_string(), "video/3gpp".to_string()),
    ("adp".to_string(), "audio/adpcm".to_string()),
    ("amr".to_string(), "audio/amr".to_string()),
    ("au".to_string(), "audio/basic".to_string()),
    ("snd".to_string(), "audio/basic".to_string()),
    ("mid".to_string(), "audio/midi".to_string()),
    ("midi".to_string(), "audio/midi".to_string()),
    ("kar".to_string(), "audio/midi".to_string()),
    ("rmi".to_string(), "audio/midi".to_string()),
    ("mxmf".to_string(), "audio/mobile-xmf".to_string()),
    ("mp3".to_string(), "audio/mpeg".to_string()),
    ("m4a".to_string(), "audio/mp4".to_string()),
    ("mp4a".to_string(), "audio/mp4".to_string()),
    ("mpga".to_string(), "audio/mpeg".to_string()),
    ("mp2".to_string(), "audio/mpeg".to_string()),
    ("mp2a".to_string(), "audio/mpeg".to_string()),
    ("m2a".to_string(), "audio/mpeg".to_string()),
    ("m3a".to_string(), "audio/mpeg".to_string()),
    ("oga".to_string(), "audio/ogg".to_string()),
    ("ogg".to_string(), "audio/ogg".to_string()),
    ("spx".to_string(), "audio/ogg".to_string()),
    ("opus".to_string(), "audio/ogg".to_string()),
    ("s3m".to_string(), "audio/s3m".to_string()),
    ("sil".to_string(), "audio/silk".to_string()),
    ("uva".to_string(), "audio/vnd.dece.audio".to_string()),
    ("uvva".to_string(), "audio/vnd.dece.audio".to_string()),
    ("eol".to_string(), "audio/vnd.digital-winds".to_string()),
    ("dra".to_string(), "audio/vnd.dra".to_string()),
    ("dts".to_string(), "audio/vnd.dts".to_string()),
    ("dtshd".to_string(), "audio/vnd.dts.hd".to_string()),
    ("lvp".to_string(), "audio/vnd.lucent.voice".to_string()),
    (
      "pya".to_string(),
      "audio/vnd.ms-playready.media.pya".to_string(),
    ),
    (
      "ecelp4800".to_string(),
      "audio/vnd.nuera.ecelp4800".to_string(),
    ),
    (
      "ecelp7470".to_string(),
      "audio/vnd.nuera.ecelp7470".to_string(),
    ),
    (
      "ecelp9600".to_string(),
      "audio/vnd.nuera.ecelp9600".to_string(),
    ),
    ("rip".to_string(), "audio/vnd.rip".to_string()),
    ("wav".to_string(), "audio/wave".to_string()),
    ("weba".to_string(), "audio/webm".to_string()),
    ("aac".to_string(), "audio/x-aac".to_string()),
    ("aif".to_string(), "audio/x-aiff".to_string()),
    ("aiff".to_string(), "audio/x-aiff".to_string()),
    ("aifc".to_string(), "audio/x-aiff".to_string()),
    ("caf".to_string(), "audio/x-caf".to_string()),
    ("flac".to_string(), "audio/x-flac".to_string()),
    ("mka".to_string(), "audio/x-matroska".to_string()),
    ("m3u".to_string(), "audio/x-mpegurl".to_string()),
    ("wax".to_string(), "audio/x-ms-wax".to_string()),
    ("wma".to_string(), "audio/x-ms-wma".to_string()),
    ("ram".to_string(), "audio/x-pn-realaudio".to_string()),
    ("ra".to_string(), "audio/x-pn-realaudio".to_string()),
    ("rmp".to_string(), "audio/x-pn-realaudio-plugin".to_string()),
    ("xm".to_string(), "audio/xm".to_string()),
    ("cdx".to_string(), "chemical/x-cdx".to_string()),
    ("cif".to_string(), "chemical/x-cif".to_string()),
    ("cmdf".to_string(), "chemical/x-cmdf".to_string()),
    ("cml".to_string(), "chemical/x-cml".to_string()),
    ("csml".to_string(), "chemical/x-csml".to_string()),
    ("xyz".to_string(), "chemical/x-xyz".to_string()),
    ("ttc".to_string(), "font/collection".to_string()),
    ("otf".to_string(), "font/otf".to_string()),
    ("ttf".to_string(), "font/ttf".to_string()),
    ("woff".to_string(), "font/woff".to_string()),
    ("woff2".to_string(), "font/woff2".to_string()),
    ("exr".to_string(), "image/aces".to_string()),
    ("apng".to_string(), "image/apng".to_string()),
    ("avci".to_string(), "image/avci".to_string()),
    ("avcs".to_string(), "image/avcs".to_string()),
    ("avif".to_string(), "image/avif".to_string()),
    ("bmp".to_string(), "image/bmp".to_string()),
    ("cgm".to_string(), "image/cgm".to_string()),
    ("drle".to_string(), "image/dicom-rle".to_string()),
    ("fits".to_string(), "image/fits".to_string()),
    ("g3".to_string(), "image/g3fax".to_string()),
    ("gif".to_string(), "image/gif".to_string()),
    ("heic".to_string(), "image/heic".to_string()),
    ("heics".to_string(), "image/heic-sequence".to_string()),
    ("heif".to_string(), "image/heif".to_string()),
    ("heifs".to_string(), "image/heif-sequence".to_string()),
    ("hej2".to_string(), "image/hej2k".to_string()),
    ("hsj2".to_string(), "image/hsj2".to_string()),
    ("ief".to_string(), "image/ief".to_string()),
    ("jls".to_string(), "image/jls".to_string()),
    ("jp2".to_string(), "image/jp2".to_string()),
    ("jpg2".to_string(), "image/jp2".to_string()),
    ("jpeg".to_string(), "image/jpeg".to_string()),
    ("jpg".to_string(), "image/jpeg".to_string()),
    ("jpe".to_string(), "image/jpeg".to_string()),
    ("jph".to_string(), "image/jph".to_string()),
    ("jhc".to_string(), "image/jphc".to_string()),
    ("jpm".to_string(), "image/jpm".to_string()),
    ("jpx".to_string(), "image/jpx".to_string()),
    ("jpf".to_string(), "image/jpx".to_string()),
    ("jxr".to_string(), "image/jxr".to_string()),
    ("jxra".to_string(), "image/jxra".to_string()),
    ("jxrs".to_string(), "image/jxrs".to_string()),
    ("jxs".to_string(), "image/jxs".to_string()),
    ("jxsc".to_string(), "image/jxsc".to_string()),
    ("jxsi".to_string(), "image/jxsi".to_string()),
    ("jxss".to_string(), "image/jxss".to_string()),
    ("ktx".to_string(), "image/ktx".to_string()),
    ("ktx2".to_string(), "image/ktx2".to_string()),
    ("png".to_string(), "image/png".to_string()),
    ("btif".to_string(), "image/prs.btif".to_string()),
    ("pti".to_string(), "image/prs.pti".to_string()),
    ("sgi".to_string(), "image/sgi".to_string()),
    ("svg".to_string(), "image/svg+xml".to_string()),
    ("svgz".to_string(), "image/svg+xml".to_string()),
    ("t38".to_string(), "image/t38".to_string()),
    ("tif".to_string(), "image/tiff".to_string()),
    ("tiff".to_string(), "image/tiff".to_string()),
    ("tfx".to_string(), "image/tiff-fx".to_string()),
    ("psd".to_string(), "image/vnd.adobe.photoshop".to_string()),
    (
      "azv".to_string(),
      "image/vnd.airzip.accelerator.azv".to_string(),
    ),
    ("uvi".to_string(), "image/vnd.dece.graphic".to_string()),
    ("uvvi".to_string(), "image/vnd.dece.graphic".to_string()),
    ("uvg".to_string(), "image/vnd.dece.graphic".to_string()),
    ("uvvg".to_string(), "image/vnd.dece.graphic".to_string()),
    ("djvu".to_string(), "image/vnd.djvu".to_string()),
    ("djv".to_string(), "image/vnd.djvu".to_string()),
    ("sub".to_string(), "text/vnd.dvb.subtitle".to_string()),
    ("dwg".to_string(), "image/vnd.dwg".to_string()),
    ("dxf".to_string(), "image/vnd.dxf".to_string()),
    ("fbs".to_string(), "image/vnd.fastbidsheet".to_string()),
    ("fpx".to_string(), "image/vnd.fpx".to_string()),
    ("fst".to_string(), "image/vnd.fst".to_string()),
    (
      "mmr".to_string(),
      "image/vnd.fujixerox.edmics-mmr".to_string(),
    ),
    (
      "rlc".to_string(),
      "image/vnd.fujixerox.edmics-rlc".to_string(),
    ),
    ("ico".to_string(), "image/vnd.microsoft.icon".to_string()),
    ("dds".to_string(), "image/vnd.ms-dds".to_string()),
    ("mdi".to_string(), "image/vnd.ms-modi".to_string()),
    ("wdp".to_string(), "image/vnd.ms-photo".to_string()),
    ("npx".to_string(), "image/vnd.net-fpx".to_string()),
    ("b16".to_string(), "image/vnd.pco.b16".to_string()),
    ("tap".to_string(), "image/vnd.tencent.tap".to_string()),
    (
      "vtf".to_string(),
      "image/vnd.valve.source.texture".to_string(),
    ),
    ("wbmp".to_string(), "image/vnd.wap.wbmp".to_string()),
    ("xif".to_string(), "image/vnd.xiff".to_string()),
    ("pcx".to_string(), "image/vnd.zbrush.pcx".to_string()),
    ("webp".to_string(), "image/webp".to_string()),
    ("3ds".to_string(), "image/x-3ds".to_string()),
    ("ras".to_string(), "image/x-cmu-raster".to_string()),
    ("cmx".to_string(), "image/x-cmx".to_string()),
    ("fh".to_string(), "image/x-freehand".to_string()),
    ("fhc".to_string(), "image/x-freehand".to_string()),
    ("fh4".to_string(), "image/x-freehand".to_string()),
    ("fh5".to_string(), "image/x-freehand".to_string()),
    ("fh7".to_string(), "image/x-freehand".to_string()),
    ("jng".to_string(), "image/x-jng".to_string()),
    ("sid".to_string(), "image/x-mrsid-image".to_string()),
    ("pic".to_string(), "image/x-pict".to_string()),
    ("pct".to_string(), "image/x-pict".to_string()),
    ("pnm".to_string(), "image/x-portable-anymap".to_string()),
    ("pbm".to_string(), "image/x-portable-bitmap".to_string()),
    ("pgm".to_string(), "image/x-portable-graymap".to_string()),
    ("ppm".to_string(), "image/x-portable-pixmap".to_string()),
    ("rgb".to_string(), "image/x-rgb".to_string()),
    ("tga".to_string(), "image/x-tga".to_string()),
    ("xbm".to_string(), "image/x-xbitmap".to_string()),
    ("xpm".to_string(), "image/x-xpixmap".to_string()),
    ("xwd".to_string(), "image/x-xwindowdump".to_string()),
    (
      "disposition-notification".to_string(),
      "message/disposition-notification".to_string(),
    ),
    ("u8msg".to_string(), "message/global".to_string()),
    (
      "u8dsn".to_string(),
      "message/global-delivery-status".to_string(),
    ),
    (
      "u8mdn".to_string(),
      "message/global-disposition-notification".to_string(),
    ),
    ("u8hdr".to_string(), "message/global-headers".to_string()),
    ("eml".to_string(), "message/rfc822".to_string()),
    ("mime".to_string(), "message/rfc822".to_string()),
    ("wsc".to_string(), "message/vnd.wfa.wsc".to_string()),
    ("3mf".to_string(), "model/3mf".to_string()),
    ("gltf".to_string(), "model/gltf+json".to_string()),
    ("glb".to_string(), "model/gltf-binary".to_string()),
    ("igs".to_string(), "model/iges".to_string()),
    ("iges".to_string(), "model/iges".to_string()),
    ("msh".to_string(), "model/mesh".to_string()),
    ("mesh".to_string(), "model/mesh".to_string()),
    ("silo".to_string(), "model/mesh".to_string()),
    ("mtl".to_string(), "model/mtl".to_string()),
    ("stpx".to_string(), "model/step+xml".to_string()),
    ("stpz".to_string(), "model/step+zip".to_string()),
    ("stpxz".to_string(), "model/step-xml+zip".to_string()),
    ("dae".to_string(), "model/vnd.collada+xml".to_string()),
    ("dwf".to_string(), "model/vnd.dwf".to_string()),
    ("gdl".to_string(), "model/vnd.gdl".to_string()),
    ("gtw".to_string(), "model/vnd.gtw".to_string()),
    ("mts".to_string(), "model/vnd.mts".to_string()),
    ("ogex".to_string(), "model/vnd.opengex".to_string()),
    (
      "x_b".to_string(),
      "model/vnd.parasolid.transmit.binary".to_string(),
    ),
    (
      "x_t".to_string(),
      "model/vnd.parasolid.transmit.text".to_string(),
    ),
    ("vds".to_string(), "model/vnd.sap.vds".to_string()),
    ("usdz".to_string(), "model/vnd.usdz+zip".to_string()),
    (
      "bsp".to_string(),
      "model/vnd.valve.source.compiled-map".to_string(),
    ),
    ("vtu".to_string(), "model/vnd.vtu".to_string()),
    ("wrl".to_string(), "model/vrml".to_string()),
    ("vrml".to_string(), "model/vrml".to_string()),
    ("x3db".to_string(), "model/x3d+fastinfoset".to_string()),
    ("x3dbz".to_string(), "model/x3d+binary".to_string()),
    ("x3dv".to_string(), "model/x3d-vrml".to_string()),
    ("x3dvz".to_string(), "model/x3d+vrml".to_string()),
    ("x3d".to_string(), "model/x3d+xml".to_string()),
    ("x3dz".to_string(), "model/x3d+xml".to_string()),
    ("appcache".to_string(), "text/cache-manifest".to_string()),
    ("manifest".to_string(), "text/cache-manifest".to_string()),
    ("ics".to_string(), "text/calendar".to_string()),
    ("ifb".to_string(), "text/calendar".to_string()),
    ("coffee".to_string(), "text/coffeescript".to_string()),
    ("litcoffee".to_string(), "text/coffeescript".to_string()),
    ("css".to_string(), "text/css".to_string()),
    ("csv".to_string(), "text/csv".to_string()),
    ("html".to_string(), "text/html".to_string()),
    ("htm".to_string(), "text/html".to_string()),
    ("shtml".to_string(), "text/html".to_string()),
    ("jade".to_string(), "text/jade".to_string()),
    ("jsx".to_string(), "text/jsx".to_string()),
    ("less".to_string(), "text/less".to_string()),
    ("markdown".to_string(), "text/markdown".to_string()),
    ("md".to_string(), "text/markdown".to_string()),
    ("mml".to_string(), "text/mathml".to_string()),
    ("mdx".to_string(), "text/mdx".to_string()),
    ("n3".to_string(), "text/n3".to_string()),
    ("txt".to_string(), "text/plain".to_string()),
    ("text".to_string(), "text/plain".to_string()),
    ("conf".to_string(), "text/plain".to_string()),
    ("def".to_string(), "text/plain".to_string()),
    ("list".to_string(), "text/plain".to_string()),
    ("log".to_string(), "text/plain".to_string()),
    ("in".to_string(), "text/plain".to_string()),
    ("ini".to_string(), "text/plain".to_string()),
    ("dsc".to_string(), "text/prs.lines.tag".to_string()),
    ("rtx".to_string(), "text/richtext".to_string()),
    ("sgml".to_string(), "text/sgml".to_string()),
    ("sgm".to_string(), "text/sgml".to_string()),
    ("shex".to_string(), "text/shex".to_string()),
    ("slim".to_string(), "text/slim".to_string()),
    ("slm".to_string(), "text/slim".to_string()),
    ("spdx".to_string(), "text/spdx".to_string()),
    ("stylus".to_string(), "text/stylus".to_string()),
    ("styl".to_string(), "text/stylus".to_string()),
    ("tsv".to_string(), "text/tab-separated-values".to_string()),
    ("t".to_string(), "text/troff".to_string()),
    ("tr".to_string(), "text/troff".to_string()),
    ("roff".to_string(), "text/troff".to_string()),
    ("man".to_string(), "text/troff".to_string()),
    ("me".to_string(), "text/troff".to_string()),
    ("ms".to_string(), "text/troff".to_string()),
    ("ttl".to_string(), "text/turtle".to_string()),
    ("uri".to_string(), "text/uri-list".to_string()),
    ("uris".to_string(), "text/uri-list".to_string()),
    ("urls".to_string(), "text/uri-list".to_string()),
    ("vcard".to_string(), "text/vcard".to_string()),
    ("curl".to_string(), "text/vnd.curl".to_string()),
    ("dcurl".to_string(), "text/vnd.curl.dcurl".to_string()),
    ("mcurl".to_string(), "text/vnd.curl.mcurl".to_string()),
    ("scurl".to_string(), "text/vnd.curl.scurl".to_string()),
    (
      "ged".to_string(),
      "text/vnd.familysearch.gedcom".to_string(),
    ),
    ("fly".to_string(), "text/vnd.fly".to_string()),
    ("flx".to_string(), "text/vnd.fmi.flexstor".to_string()),
    ("gv".to_string(), "text/vnd.graphviz".to_string()),
    ("3dml".to_string(), "text/vnd.in3d.3dml".to_string()),
    ("spot".to_string(), "text/vnd.in3d.spot".to_string()),
    (
      "jad".to_string(),
      "text/vnd.sun.j2me.app-descriptor".to_string(),
    ),
    ("wml".to_string(), "text/vnd.wap.wml".to_string()),
    ("wmls".to_string(), "text/vnd.wap.wmlscript".to_string()),
    ("vtt".to_string(), "text/vtt".to_string()),
    ("s".to_string(), "text/x-asm".to_string()),
    ("asm".to_string(), "text/x-asm".to_string()),
    ("c".to_string(), "text/x-c".to_string()),
    ("cc".to_string(), "text/x-c".to_string()),
    ("cxx".to_string(), "text/x-c".to_string()),
    ("cpp".to_string(), "text/x-c".to_string()),
    ("h".to_string(), "text/x-c".to_string()),
    ("hh".to_string(), "text/x-c".to_string()),
    ("dic".to_string(), "text/x-c".to_string()),
    ("htc".to_string(), "text/x-component".to_string()),
    ("f".to_string(), "text/x-fortran".to_string()),
    ("for".to_string(), "text/x-fortran".to_string()),
    ("f77".to_string(), "text/x-fortran".to_string()),
    ("f90".to_string(), "text/x-fortran".to_string()),
    ("hbs".to_string(), "text/x-handlebars-template".to_string()),
    ("java".to_string(), "text/x-java-source".to_string()),
    ("lua".to_string(), "text/x-lua".to_string()),
    ("mkd".to_string(), "text/x-markdown".to_string()),
    ("nfo".to_string(), "text/x-nfo".to_string()),
    ("opml".to_string(), "text/x-opml".to_string()),
    ("p".to_string(), "text/x-pascal".to_string()),
    ("pas".to_string(), "text/x-pascal".to_string()),
    ("pde".to_string(), "text/x-processing".to_string()),
    ("sass".to_string(), "text/x-sass".to_string()),
    ("scss".to_string(), "text/x-scss".to_string()),
    ("etx".to_string(), "text/x-setext".to_string()),
    ("sfv".to_string(), "text/x-sfv".to_string()),
    ("ymp".to_string(), "text/x-suse-ymp".to_string()),
    ("uu".to_string(), "text/x-uuencode".to_string()),
    ("vcs".to_string(), "text/x-vcalendar".to_string()),
    ("vcf".to_string(), "text/x-vcard".to_string()),
    ("yaml".to_string(), "text/yaml".to_string()),
    ("yml".to_string(), "text/yaml".to_string()),
    ("3gp".to_string(), "video/3gpp".to_string()),
    ("3g2".to_string(), "video/3gpp2".to_string()),
    ("h261".to_string(), "video/h261".to_string()),
    ("h263".to_string(), "video/h263".to_string()),
    ("h264".to_string(), "video/h264".to_string()),
    ("m4s".to_string(), "video/iso.segment".to_string()),
    ("jpgv".to_string(), "video/jpeg".to_string()),
    ("jpgm".to_string(), "video/jpm".to_string()),
    ("mj2".to_string(), "video/mj2".to_string()),
    ("mjp2".to_string(), "video/mj2".to_string()),
    ("ts".to_string(), "video/mp2t".to_string()),
    ("mp4".to_string(), "video/mp4".to_string()),
    ("mp4v".to_string(), "video/mp4".to_string()),
    ("mpg4".to_string(), "video/mp4".to_string()),
    ("mpeg".to_string(), "video/mpeg".to_string()),
    ("mpg".to_string(), "video/mpeg".to_string()),
    ("mpe".to_string(), "video/mpeg".to_string()),
    ("m1v".to_string(), "video/mpeg".to_string()),
    ("m2v".to_string(), "video/mpeg".to_string()),
    ("ogv".to_string(), "video/ogg".to_string()),
    ("qt".to_string(), "video/quicktime".to_string()),
    ("mov".to_string(), "video/quicktime".to_string()),
    ("uvh".to_string(), "video/vnd.dece.hd".to_string()),
    ("uvvh".to_string(), "video/vnd.dece.hd".to_string()),
    ("uvm".to_string(), "video/vnd.dece.mobile".to_string()),
    ("uvvm".to_string(), "video/vnd.dece.mobile".to_string()),
    ("uvp".to_string(), "video/vnd.dece.pd".to_string()),
    ("uvvp".to_string(), "video/vnd.dece.pd".to_string()),
    ("uvs".to_string(), "video/vnd.dece.sd".to_string()),
    ("uvvs".to_string(), "video/vnd.dece.sd".to_string()),
    ("uvv".to_string(), "video/vnd.dece.video".to_string()),
    ("uvvv".to_string(), "video/vnd.dece.video".to_string()),
    ("dvb".to_string(), "video/vnd.dvb.file".to_string()),
    ("fvt".to_string(), "video/vnd.fvt".to_string()),
    ("mxu".to_string(), "video/vnd.mpegurl".to_string()),
    ("m4u".to_string(), "video/vnd.mpegurl".to_string()),
    (
      "pyv".to_string(),
      "video/vnd.ms-playready.media.pyv".to_string(),
    ),
    ("uvu".to_string(), "video/vnd.uvvu.mp4".to_string()),
    ("uvvu".to_string(), "video/vnd.uvvu.mp4".to_string()),
    ("viv".to_string(), "video/vnd.vivo".to_string()),
    ("webm".to_string(), "video/webm".to_string()),
    ("f4v".to_string(), "video/x-f4v".to_string()),
    ("fli".to_string(), "video/x-fli".to_string()),
    ("flv".to_string(), "video/x-flv".to_string()),
    ("m4v".to_string(), "video/x-m4v".to_string()),
    ("mkv".to_string(), "video/x-matroska".to_string()),
    ("mk3d".to_string(), "video/x-matroska".to_string()),
    ("mks".to_string(), "video/x-matroska".to_string()),
    ("mng".to_string(), "video/x-mng".to_string()),
    ("asf".to_string(), "video/x-ms-asf".to_string()),
    ("asx".to_string(), "video/x-ms-asf".to_string()),
    ("vob".to_string(), "video/x-ms-vob".to_string()),
    ("wm".to_string(), "video/x-ms-wm".to_string()),
    ("wmv".to_string(), "video/x-ms-wmv".to_string()),
    ("wmx".to_string(), "video/x-ms-wmx".to_string()),
    ("wvx".to_string(), "video/x-ms-wvx".to_string()),
    ("avi".to_string(), "video/x-msvideo".to_string()),
    ("movie".to_string(), "video/x-sgi-movie".to_string()),
    ("smv".to_string(), "video/x-smv".to_string()),
    ("ice".to_string(), "x-conference/x-cooltalk".to_string()),
  ])
});
