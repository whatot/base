  


<!DOCTYPE html>
<html>
  <head prefix="og: http://ogp.me/ns# fb: http://ogp.me/ns/fb# githubog: http://ogp.me/ns/fb/githubog#">
    <meta charset='utf-8'>
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
        <title>tools/SimpleHTTPServerWithUpload.py at master · tualatrix/tools</title>
    <link rel="search" type="application/opensearchdescription+xml" href="/opensearch.xml" title="GitHub" />
    <link rel="fluid-icon" href="https://github.com/fluidicon.png" title="GitHub" />
    <link rel="apple-touch-icon" sizes="57x57" href="/apple-touch-icon-114.png" />
    <link rel="apple-touch-icon" sizes="114x114" href="/apple-touch-icon-114.png" />
    <link rel="apple-touch-icon" sizes="72x72" href="/apple-touch-icon-144.png" />
    <link rel="apple-touch-icon" sizes="144x144" href="/apple-touch-icon-144.png" />
    <link rel="logo" type="image/svg" href="http://github-media-downloads.s3.amazonaws.com/github-logo.svg" />
    <link rel="xhr-socket" href="/_sockets">
    <meta name="msapplication-TileImage" content="/windows-tile.png">
    <meta name="msapplication-TileColor" content="#ffffff">

    
    
    <link rel="icon" type="image/x-icon" href="/favicon.ico" />

    <meta content="authenticity_token" name="csrf-param" />
<meta content="gKWSvSMtaCob7HcZgYTna8BCsoz73A5XDiuG9jMEz1I=" name="csrf-token" />

    <link href="https://a248.e.akamai.net/assets.github.com/assets/github-e1db8fdb716d63e92ba830ea0991d4b45deee768.css" media="all" rel="stylesheet" type="text/css" />
    <link href="https://a248.e.akamai.net/assets.github.com/assets/github2-c15137b0b05c94db05fa047ecd589d7a7df41d85.css" media="all" rel="stylesheet" type="text/css" />
    


      <script src="https://a248.e.akamai.net/assets.github.com/assets/frameworks-010d500708696b4ecee44478b5229d626367e844.js" type="text/javascript"></script>
      <script src="https://a248.e.akamai.net/assets.github.com/assets/github-d0f76964f2a7dfb9b87493f279d45019ffeeec05.js" type="text/javascript"></script>
      
      <meta http-equiv="x-pjax-version" content="ef621fa1d352e7b33a37c96aad954673">

        <link data-pjax-transient rel='permalink' href='/tualatrix/tools/blob/1f0e56938e09ae66b19756b5a98d578201cbb3d9/SimpleHTTPServerWithUpload.py'>
    <meta property="og:title" content="tools"/>
    <meta property="og:type" content="githubog:gitrepository"/>
    <meta property="og:url" content="https://github.com/tualatrix/tools"/>
    <meta property="og:image" content="https://secure.gravatar.com/avatar/3aa394e5aef22274b9d36a74adb787e9?s=420&amp;d=https://a248.e.akamai.net/assets.github.com%2Fimages%2Fgravatars%2Fgravatar-user-420.png"/>
    <meta property="og:site_name" content="GitHub"/>
    <meta property="og:description" content="A lot of my tools"/>
    <meta property="twitter:card" content="summary"/>
    <meta property="twitter:site" content="@GitHub">
    <meta property="twitter:title" content="tualatrix/tools"/>

    <meta name="description" content="A lot of my tools" />

  <link href="https://github.com/tualatrix/tools/commits/master.atom" rel="alternate" title="Recent Commits to tools:master" type="application/atom+xml" />

  </head>


  <body class="logged_in page-blob linux vis-public env-production  ">
    <div id="wrapper">

      

      
      
      

      <div class="header header-logged-in true">
  <div class="container clearfix">

    <a class="header-logo-invertocat" href="https://github.com/">
  <span class="mega-icon mega-icon-invertocat"></span>
</a>

    <div class="divider-vertical"></div>

      <a href="/notifications" class="notification-indicator tooltipped downwards" title="You have no unread notifications">
    <span class="mail-status all-read"></span>
  </a>
  <div class="divider-vertical"></div>


      <div class="command-bar js-command-bar  ">
            <form accept-charset="UTF-8" action="/search" class="command-bar-form" id="top_search_form" method="get">
  <a href="/search/advanced" class="advanced-search-icon tooltipped downwards command-bar-search" id="advanced_search" title="Advanced search"><span class="mini-icon mini-icon-advanced-search "></span></a>

  <input type="text" data-hotkey="/ s" name="q" id="js-command-bar-field" placeholder="Search or type a command" tabindex="1" data-username="whatot" autocapitalize="off">

  <span class="mini-icon help tooltipped downwards" title="Show command bar help">
    <span class="mini-icon mini-icon-help"></span>
  </span>

  <input type="hidden" name="ref" value="cmdform">

    <input type="hidden" class="js-repository-name-with-owner" value="tualatrix/tools"/>
    <input type="hidden" class="js-repository-branch" value="master"/>
    <input type="hidden" class="js-repository-tree-sha" value="4e923d6597da1c991175d579bd88c91e8b13668a"/>

  <div class="divider-vertical"></div>
</form>
        <ul class="top-nav">
            <li class="explore"><a href="https://github.com/explore">Explore</a></li>
            <li><a href="https://gist.github.com">Gist</a></li>
            <li><a href="/blog">Blog</a></li>
          <li><a href="http://help.github.com">Help</a></li>
        </ul>
      </div>

    

  

    <ul id="user-links">
      <li>
        <a href="https://github.com/whatot" class="name">
          <img height="20" src="https://secure.gravatar.com/avatar/47b31bb5ba60faac91511f87fbae3787?s=140&amp;d=https://a248.e.akamai.net/assets.github.com%2Fimages%2Fgravatars%2Fgravatar-user-420.png" width="20" /> whatot
        </a>
      </li>

        <li>
          <a href="/new" id="new_repo" class="tooltipped downwards" title="Create a new repo">
            <span class="mini-icon mini-icon-create"></span>
          </a>
        </li>

        <li>
          <a href="/settings/profile" id="account_settings"
            class="tooltipped downwards"
            title="Account settings ">
            <span class="mini-icon mini-icon-account-settings"></span>
          </a>
        </li>
        <li>
          <a class="tooltipped downwards" href="/logout" data-method="post" id="logout" title="Sign out">
            <span class="mini-icon mini-icon-logout"></span>
          </a>
        </li>

    </ul>


<div class="js-new-dropdown-contents hidden">
  <ul class="dropdown-menu">
    <li>
      <a href="/new"><span class="mini-icon mini-icon-create"></span> New repository</a>
    </li>
    <li>
        <a href="https://github.com/tualatrix/tools/issues/new"><span class="mini-icon mini-icon-issue-opened"></span> New issue</a>
    </li>
    <li>
    </li>
    <li>
      <a href="/organizations/new"><span class="mini-icon mini-icon-u-list"></span> New organization</a>
    </li>
  </ul>
</div>


    
  </div>
</div>

      

      

      


            <div class="site hfeed" itemscope itemtype="http://schema.org/WebPage">
      <div class="hentry">
        
        <div class="pagehead repohead instapaper_ignore readability-menu ">
          <div class="container">
            <div class="title-actions-bar">
              


<ul class="pagehead-actions">


    <li class="subscription">
      <form accept-charset="UTF-8" action="/notifications/subscribe" data-autosubmit="true" data-remote="true" method="post"><div style="margin:0;padding:0;display:inline"><input name="authenticity_token" type="hidden" value="gKWSvSMtaCob7HcZgYTna8BCsoz73A5XDiuG9jMEz1I=" /></div>  <input id="repository_id" name="repository_id" type="hidden" value="450814" />

    <div class="select-menu js-menu-container js-select-menu">
      <span class="minibutton select-menu-button js-menu-target">
        <span class="js-select-button">
          <span class="mini-icon mini-icon-watching"></span>
          Watch
        </span>
      </span>

      <div class="select-menu-modal-holder js-menu-content">
        <div class="select-menu-modal">
          <div class="select-menu-header">
            <span class="select-menu-title">Notification status</span>
            <span class="mini-icon mini-icon-remove-close js-menu-close"></span>
          </div> <!-- /.select-menu-header -->

          <div class="select-menu-list js-navigation-container">

            <div class="select-menu-item js-navigation-item js-navigation-target selected">
              <span class="select-menu-item-icon mini-icon mini-icon-confirm"></span>
              <div class="select-menu-item-text">
                <input checked="checked" id="do_included" name="do" type="radio" value="included" />
                <h4>Not watching</h4>
                <span class="description">You only receive notifications for discussions in which you participate or are @mentioned.</span>
                <span class="js-select-button-text hidden-select-button-text">
                  <span class="mini-icon mini-icon-watching"></span>
                  Watch
                </span>
              </div>
            </div> <!-- /.select-menu-item -->

            <div class="select-menu-item js-navigation-item js-navigation-target ">
              <span class="select-menu-item-icon mini-icon mini-icon-confirm"></span>
              <div class="select-menu-item-text">
                <input id="do_subscribed" name="do" type="radio" value="subscribed" />
                <h4>Watching</h4>
                <span class="description">You receive notifications for all discussions in this repository.</span>
                <span class="js-select-button-text hidden-select-button-text">
                  <span class="mini-icon mini-icon-unwatch"></span>
                  Unwatch
                </span>
              </div>
            </div> <!-- /.select-menu-item -->

            <div class="select-menu-item js-navigation-item js-navigation-target ">
              <span class="select-menu-item-icon mini-icon mini-icon-confirm"></span>
              <div class="select-menu-item-text">
                <input id="do_ignore" name="do" type="radio" value="ignore" />
                <h4>Ignoring</h4>
                <span class="description">You do not receive any notifications for discussions in this repository.</span>
                <span class="js-select-button-text hidden-select-button-text">
                  <span class="mini-icon mini-icon-mute"></span>
                  Stop ignoring
                </span>
              </div>
            </div> <!-- /.select-menu-item -->

          </div> <!-- /.select-menu-list -->

        </div> <!-- /.select-menu-modal -->
      </div> <!-- /.select-menu-modal-holder -->
    </div> <!-- /.select-menu -->

</form>
    </li>

    <li class="js-toggler-container js-social-container starring-container ">
      <a href="/tualatrix/tools/unstar" class="minibutton js-toggler-target star-button starred upwards" title="Unstar this repo" data-remote="true" data-method="post" rel="nofollow">
        <span class="mini-icon mini-icon-remove-star"></span>
        <span class="text">Unstar</span>
      </a>
      <a href="/tualatrix/tools/star" class="minibutton js-toggler-target star-button unstarred upwards" title="Star this repo" data-remote="true" data-method="post" rel="nofollow">
        <span class="mini-icon mini-icon-star"></span>
        <span class="text">Star</span>
      </a>
      <a class="social-count js-social-count" href="/tualatrix/tools/stargazers">22</a>
    </li>

        <li>
          <a href="/tualatrix/tools/fork" class="minibutton js-toggler-target fork-button lighter upwards" title="Fork this repo" rel="nofollow" data-method="post">
            <span class="mini-icon mini-icon-branch-create"></span>
            <span class="text">Fork</span>
          </a>
          <a href="/tualatrix/tools/network" class="social-count">5</a>
        </li>


</ul>

              <h1 itemscope itemtype="http://data-vocabulary.org/Breadcrumb" class="entry-title public">
                <span class="repo-label"><span>public</span></span>
                <span class="mega-icon mega-icon-public-repo"></span>
                <span class="author vcard">
                  <a href="/tualatrix" class="url fn" itemprop="url" rel="author">
                  <span itemprop="title">tualatrix</span>
                  </a></span> /
                <strong><a href="/tualatrix/tools" class="js-current-repository">tools</a></strong>
              </h1>
            </div>

            
  <ul class="tabs">
    <li><a href="/tualatrix/tools" class="selected" highlight="repo_source repo_downloads repo_commits repo_tags repo_branches">Code</a></li>
    <li><a href="/tualatrix/tools/network" highlight="repo_network">Network</a></li>
    <li><a href="/tualatrix/tools/pulls" highlight="repo_pulls">Pull Requests <span class='counter'>0</span></a></li>

      <li><a href="/tualatrix/tools/issues" highlight="repo_issues">Issues <span class='counter'>0</span></a></li>

      <li><a href="/tualatrix/tools/wiki" highlight="repo_wiki">Wiki</a></li>


    <li><a href="/tualatrix/tools/graphs" highlight="repo_graphs repo_contributors">Graphs</a></li>


  </ul>
  
<div class="tabnav">

  <span class="tabnav-right">
    <ul class="tabnav-tabs">
          <li><a href="/tualatrix/tools/tags" class="tabnav-tab" highlight="repo_tags">Tags <span class="counter blank">0</span></a></li>
    </ul>
    
  </span>

  <div class="tabnav-widget scope">


    <div class="select-menu js-menu-container js-select-menu js-branch-menu">
      <a class="minibutton select-menu-button js-menu-target" data-hotkey="w" data-ref="master">
        <span class="mini-icon mini-icon-branch"></span>
        <i>branch:</i>
        <span class="js-select-button">master</span>
      </a>

      <div class="select-menu-modal-holder js-menu-content js-navigation-container">

        <div class="select-menu-modal">
          <div class="select-menu-header">
            <span class="select-menu-title">Switch branches/tags</span>
            <span class="mini-icon mini-icon-remove-close js-menu-close"></span>
          </div> <!-- /.select-menu-header -->

          <div class="select-menu-filters">
            <div class="select-menu-text-filter">
              <input type="text" id="commitish-filter-field" class="js-filterable-field js-navigation-enable" placeholder="Filter branches/tags">
            </div>
            <div class="select-menu-tabs">
              <ul>
                <li class="select-menu-tab">
                  <a href="#" data-tab-filter="branches" class="js-select-menu-tab">Branches</a>
                </li>
                <li class="select-menu-tab">
                  <a href="#" data-tab-filter="tags" class="js-select-menu-tab">Tags</a>
                </li>
              </ul>
            </div><!-- /.select-menu-tabs -->
          </div><!-- /.select-menu-filters -->

          <div class="select-menu-list select-menu-tab-bucket js-select-menu-tab-bucket css-truncate" data-tab-filter="branches">

            <div data-filterable-for="commitish-filter-field" data-filterable-type="substring">

                <div class="select-menu-item js-navigation-item js-navigation-target selected">
                  <span class="select-menu-item-icon mini-icon mini-icon-confirm"></span>
                  <a href="/tualatrix/tools/blob/master/SimpleHTTPServerWithUpload.py" class="js-navigation-open select-menu-item-text js-select-button-text css-truncate-target" data-name="master" rel="nofollow" title="master">master</a>
                </div> <!-- /.select-menu-item -->
            </div>

              <div class="select-menu-no-results">Nothing to show</div>
          </div> <!-- /.select-menu-list -->


          <div class="select-menu-list select-menu-tab-bucket js-select-menu-tab-bucket css-truncate" data-tab-filter="tags">
            <div data-filterable-for="commitish-filter-field" data-filterable-type="substring">

            </div>

            <div class="select-menu-no-results">Nothing to show</div>

          </div> <!-- /.select-menu-list -->

        </div> <!-- /.select-menu-modal -->
      </div> <!-- /.select-menu-modal-holder -->
    </div> <!-- /.select-menu -->

  </div> <!-- /.scope -->

  <ul class="tabnav-tabs">
    <li><a href="/tualatrix/tools" class="selected tabnav-tab" highlight="repo_source">Files</a></li>
    <li><a href="/tualatrix/tools/commits/master" class="tabnav-tab" highlight="repo_commits">Commits</a></li>
    <li><a href="/tualatrix/tools/branches" class="tabnav-tab" highlight="repo_branches" rel="nofollow">Branches <span class="counter ">1</span></a></li>
  </ul>

</div>

  
  
  


            
          </div>
        </div><!-- /.repohead -->

        <div id="js-repo-pjax-container" class="container context-loader-container" data-pjax-container>
          


<!-- blob contrib key: blob_contributors:v21:f1889b846db816a7e576e6c31b98ca13 -->
<!-- blob contrib frag key: views10/v8/blob_contributors:v21:f1889b846db816a7e576e6c31b98ca13 -->


<div id="slider">
    <div class="frame-meta">

      <p title="This is a placeholder element" class="js-history-link-replace hidden"></p>

        <div class="breadcrumb">
          <span class='bold'><span itemscope="" itemtype="http://data-vocabulary.org/Breadcrumb"><a href="/tualatrix/tools" class="js-slide-to" data-branch="master" data-direction="back" itemscope="url"><span itemprop="title">tools</span></a></span></span><span class="separator"> / </span><strong class="final-path">SimpleHTTPServerWithUpload.py</strong> <span class="js-zeroclipboard zeroclipboard-button" data-clipboard-text="SimpleHTTPServerWithUpload.py" data-copied-hint="copied!" title="copy to clipboard"><span class="mini-icon mini-icon-clipboard"></span></span>
        </div>

      <a href="/tualatrix/tools/find/master" class="js-slide-to" data-hotkey="t" style="display:none">Show File Finder</a>


        
  <div class="commit file-history-tease">
    <img class="main-avatar" height="24" src="https://secure.gravatar.com/avatar/3aa394e5aef22274b9d36a74adb787e9?s=140&amp;d=https://a248.e.akamai.net/assets.github.com%2Fimages%2Fgravatars%2Fgravatar-user-420.png" width="24" />
    <span class="author"><a href="/tualatrix" rel="author">tualatrix</a></span>
    <time class="js-relative-date" datetime="2010-11-27T20:09:07-08:00" title="2010-11-27 20:09:07">November 27, 2010</time>
    <div class="commit-title">
        <a href="/tualatrix/tools/commit/b570198e85c26d7c29fa7155bf418961e0f8d959" class="message">Add SimpleHTTPServrWithUpload.py;</a>
    </div>

    <div class="participation">
      <p class="quickstat"><a href="#blob_contributors_box" rel="facebox"><strong>1</strong> contributor</a></p>
      
    </div>
    <div id="blob_contributors_box" style="display:none">
      <h2>Users on GitHub who have contributed to this file</h2>
      <ul class="facebox-user-list">
        <li>
          <img height="24" src="https://secure.gravatar.com/avatar/3aa394e5aef22274b9d36a74adb787e9?s=140&amp;d=https://a248.e.akamai.net/assets.github.com%2Fimages%2Fgravatars%2Fgravatar-user-420.png" width="24" />
          <a href="/tualatrix">tualatrix</a>
        </li>
      </ul>
    </div>
  </div>


    </div><!-- ./.frame-meta -->

    <div class="frames">
      <div class="frame" data-permalink-url="/tualatrix/tools/blob/1f0e56938e09ae66b19756b5a98d578201cbb3d9/SimpleHTTPServerWithUpload.py" data-title="tools/SimpleHTTPServerWithUpload.py at master · tualatrix/tools · GitHub" data-type="blob">

        <div id="files" class="bubble">
          <div class="file">
            <div class="meta">
              <div class="info">
                <span class="icon"><b class="mini-icon mini-icon-text-file"></b></span>
                <span class="mode" title="File Mode">executable file</span>
                  <span>297 lines (254 sloc)</span>
                <span>10.451 kb</span>
              </div>
              <div class="actions">
                <div class="button-group">
                        <a class="minibutton tooltipped leftwards"
                           title="Clicking this button will automatically fork this project so you can edit the file"
                           href="/tualatrix/tools/edit/master/SimpleHTTPServerWithUpload.py"
                           data-method="post" rel="nofollow">Edit</a>
                  <a href="/tualatrix/tools/raw/master/SimpleHTTPServerWithUpload.py" class="button minibutton " id="raw-url">Raw</a>
                    <a href="/tualatrix/tools/blame/master/SimpleHTTPServerWithUpload.py" class="button minibutton ">Blame</a>
                  <a href="/tualatrix/tools/commits/master/SimpleHTTPServerWithUpload.py" class="button minibutton " rel="nofollow">History</a>
                </div><!-- /.button-group -->
              </div><!-- /.actions -->

            </div>
                <div class="blob-wrapper data type-python js-blob-data">
      <table class="file-code file-diff">
        <tr class="file-code-line">
          <td class="blob-line-nums">
            <span id="L1" rel="#L1">1</span>
<span id="L2" rel="#L2">2</span>
<span id="L3" rel="#L3">3</span>
<span id="L4" rel="#L4">4</span>
<span id="L5" rel="#L5">5</span>
<span id="L6" rel="#L6">6</span>
<span id="L7" rel="#L7">7</span>
<span id="L8" rel="#L8">8</span>
<span id="L9" rel="#L9">9</span>
<span id="L10" rel="#L10">10</span>
<span id="L11" rel="#L11">11</span>
<span id="L12" rel="#L12">12</span>
<span id="L13" rel="#L13">13</span>
<span id="L14" rel="#L14">14</span>
<span id="L15" rel="#L15">15</span>
<span id="L16" rel="#L16">16</span>
<span id="L17" rel="#L17">17</span>
<span id="L18" rel="#L18">18</span>
<span id="L19" rel="#L19">19</span>
<span id="L20" rel="#L20">20</span>
<span id="L21" rel="#L21">21</span>
<span id="L22" rel="#L22">22</span>
<span id="L23" rel="#L23">23</span>
<span id="L24" rel="#L24">24</span>
<span id="L25" rel="#L25">25</span>
<span id="L26" rel="#L26">26</span>
<span id="L27" rel="#L27">27</span>
<span id="L28" rel="#L28">28</span>
<span id="L29" rel="#L29">29</span>
<span id="L30" rel="#L30">30</span>
<span id="L31" rel="#L31">31</span>
<span id="L32" rel="#L32">32</span>
<span id="L33" rel="#L33">33</span>
<span id="L34" rel="#L34">34</span>
<span id="L35" rel="#L35">35</span>
<span id="L36" rel="#L36">36</span>
<span id="L37" rel="#L37">37</span>
<span id="L38" rel="#L38">38</span>
<span id="L39" rel="#L39">39</span>
<span id="L40" rel="#L40">40</span>
<span id="L41" rel="#L41">41</span>
<span id="L42" rel="#L42">42</span>
<span id="L43" rel="#L43">43</span>
<span id="L44" rel="#L44">44</span>
<span id="L45" rel="#L45">45</span>
<span id="L46" rel="#L46">46</span>
<span id="L47" rel="#L47">47</span>
<span id="L48" rel="#L48">48</span>
<span id="L49" rel="#L49">49</span>
<span id="L50" rel="#L50">50</span>
<span id="L51" rel="#L51">51</span>
<span id="L52" rel="#L52">52</span>
<span id="L53" rel="#L53">53</span>
<span id="L54" rel="#L54">54</span>
<span id="L55" rel="#L55">55</span>
<span id="L56" rel="#L56">56</span>
<span id="L57" rel="#L57">57</span>
<span id="L58" rel="#L58">58</span>
<span id="L59" rel="#L59">59</span>
<span id="L60" rel="#L60">60</span>
<span id="L61" rel="#L61">61</span>
<span id="L62" rel="#L62">62</span>
<span id="L63" rel="#L63">63</span>
<span id="L64" rel="#L64">64</span>
<span id="L65" rel="#L65">65</span>
<span id="L66" rel="#L66">66</span>
<span id="L67" rel="#L67">67</span>
<span id="L68" rel="#L68">68</span>
<span id="L69" rel="#L69">69</span>
<span id="L70" rel="#L70">70</span>
<span id="L71" rel="#L71">71</span>
<span id="L72" rel="#L72">72</span>
<span id="L73" rel="#L73">73</span>
<span id="L74" rel="#L74">74</span>
<span id="L75" rel="#L75">75</span>
<span id="L76" rel="#L76">76</span>
<span id="L77" rel="#L77">77</span>
<span id="L78" rel="#L78">78</span>
<span id="L79" rel="#L79">79</span>
<span id="L80" rel="#L80">80</span>
<span id="L81" rel="#L81">81</span>
<span id="L82" rel="#L82">82</span>
<span id="L83" rel="#L83">83</span>
<span id="L84" rel="#L84">84</span>
<span id="L85" rel="#L85">85</span>
<span id="L86" rel="#L86">86</span>
<span id="L87" rel="#L87">87</span>
<span id="L88" rel="#L88">88</span>
<span id="L89" rel="#L89">89</span>
<span id="L90" rel="#L90">90</span>
<span id="L91" rel="#L91">91</span>
<span id="L92" rel="#L92">92</span>
<span id="L93" rel="#L93">93</span>
<span id="L94" rel="#L94">94</span>
<span id="L95" rel="#L95">95</span>
<span id="L96" rel="#L96">96</span>
<span id="L97" rel="#L97">97</span>
<span id="L98" rel="#L98">98</span>
<span id="L99" rel="#L99">99</span>
<span id="L100" rel="#L100">100</span>
<span id="L101" rel="#L101">101</span>
<span id="L102" rel="#L102">102</span>
<span id="L103" rel="#L103">103</span>
<span id="L104" rel="#L104">104</span>
<span id="L105" rel="#L105">105</span>
<span id="L106" rel="#L106">106</span>
<span id="L107" rel="#L107">107</span>
<span id="L108" rel="#L108">108</span>
<span id="L109" rel="#L109">109</span>
<span id="L110" rel="#L110">110</span>
<span id="L111" rel="#L111">111</span>
<span id="L112" rel="#L112">112</span>
<span id="L113" rel="#L113">113</span>
<span id="L114" rel="#L114">114</span>
<span id="L115" rel="#L115">115</span>
<span id="L116" rel="#L116">116</span>
<span id="L117" rel="#L117">117</span>
<span id="L118" rel="#L118">118</span>
<span id="L119" rel="#L119">119</span>
<span id="L120" rel="#L120">120</span>
<span id="L121" rel="#L121">121</span>
<span id="L122" rel="#L122">122</span>
<span id="L123" rel="#L123">123</span>
<span id="L124" rel="#L124">124</span>
<span id="L125" rel="#L125">125</span>
<span id="L126" rel="#L126">126</span>
<span id="L127" rel="#L127">127</span>
<span id="L128" rel="#L128">128</span>
<span id="L129" rel="#L129">129</span>
<span id="L130" rel="#L130">130</span>
<span id="L131" rel="#L131">131</span>
<span id="L132" rel="#L132">132</span>
<span id="L133" rel="#L133">133</span>
<span id="L134" rel="#L134">134</span>
<span id="L135" rel="#L135">135</span>
<span id="L136" rel="#L136">136</span>
<span id="L137" rel="#L137">137</span>
<span id="L138" rel="#L138">138</span>
<span id="L139" rel="#L139">139</span>
<span id="L140" rel="#L140">140</span>
<span id="L141" rel="#L141">141</span>
<span id="L142" rel="#L142">142</span>
<span id="L143" rel="#L143">143</span>
<span id="L144" rel="#L144">144</span>
<span id="L145" rel="#L145">145</span>
<span id="L146" rel="#L146">146</span>
<span id="L147" rel="#L147">147</span>
<span id="L148" rel="#L148">148</span>
<span id="L149" rel="#L149">149</span>
<span id="L150" rel="#L150">150</span>
<span id="L151" rel="#L151">151</span>
<span id="L152" rel="#L152">152</span>
<span id="L153" rel="#L153">153</span>
<span id="L154" rel="#L154">154</span>
<span id="L155" rel="#L155">155</span>
<span id="L156" rel="#L156">156</span>
<span id="L157" rel="#L157">157</span>
<span id="L158" rel="#L158">158</span>
<span id="L159" rel="#L159">159</span>
<span id="L160" rel="#L160">160</span>
<span id="L161" rel="#L161">161</span>
<span id="L162" rel="#L162">162</span>
<span id="L163" rel="#L163">163</span>
<span id="L164" rel="#L164">164</span>
<span id="L165" rel="#L165">165</span>
<span id="L166" rel="#L166">166</span>
<span id="L167" rel="#L167">167</span>
<span id="L168" rel="#L168">168</span>
<span id="L169" rel="#L169">169</span>
<span id="L170" rel="#L170">170</span>
<span id="L171" rel="#L171">171</span>
<span id="L172" rel="#L172">172</span>
<span id="L173" rel="#L173">173</span>
<span id="L174" rel="#L174">174</span>
<span id="L175" rel="#L175">175</span>
<span id="L176" rel="#L176">176</span>
<span id="L177" rel="#L177">177</span>
<span id="L178" rel="#L178">178</span>
<span id="L179" rel="#L179">179</span>
<span id="L180" rel="#L180">180</span>
<span id="L181" rel="#L181">181</span>
<span id="L182" rel="#L182">182</span>
<span id="L183" rel="#L183">183</span>
<span id="L184" rel="#L184">184</span>
<span id="L185" rel="#L185">185</span>
<span id="L186" rel="#L186">186</span>
<span id="L187" rel="#L187">187</span>
<span id="L188" rel="#L188">188</span>
<span id="L189" rel="#L189">189</span>
<span id="L190" rel="#L190">190</span>
<span id="L191" rel="#L191">191</span>
<span id="L192" rel="#L192">192</span>
<span id="L193" rel="#L193">193</span>
<span id="L194" rel="#L194">194</span>
<span id="L195" rel="#L195">195</span>
<span id="L196" rel="#L196">196</span>
<span id="L197" rel="#L197">197</span>
<span id="L198" rel="#L198">198</span>
<span id="L199" rel="#L199">199</span>
<span id="L200" rel="#L200">200</span>
<span id="L201" rel="#L201">201</span>
<span id="L202" rel="#L202">202</span>
<span id="L203" rel="#L203">203</span>
<span id="L204" rel="#L204">204</span>
<span id="L205" rel="#L205">205</span>
<span id="L206" rel="#L206">206</span>
<span id="L207" rel="#L207">207</span>
<span id="L208" rel="#L208">208</span>
<span id="L209" rel="#L209">209</span>
<span id="L210" rel="#L210">210</span>
<span id="L211" rel="#L211">211</span>
<span id="L212" rel="#L212">212</span>
<span id="L213" rel="#L213">213</span>
<span id="L214" rel="#L214">214</span>
<span id="L215" rel="#L215">215</span>
<span id="L216" rel="#L216">216</span>
<span id="L217" rel="#L217">217</span>
<span id="L218" rel="#L218">218</span>
<span id="L219" rel="#L219">219</span>
<span id="L220" rel="#L220">220</span>
<span id="L221" rel="#L221">221</span>
<span id="L222" rel="#L222">222</span>
<span id="L223" rel="#L223">223</span>
<span id="L224" rel="#L224">224</span>
<span id="L225" rel="#L225">225</span>
<span id="L226" rel="#L226">226</span>
<span id="L227" rel="#L227">227</span>
<span id="L228" rel="#L228">228</span>
<span id="L229" rel="#L229">229</span>
<span id="L230" rel="#L230">230</span>
<span id="L231" rel="#L231">231</span>
<span id="L232" rel="#L232">232</span>
<span id="L233" rel="#L233">233</span>
<span id="L234" rel="#L234">234</span>
<span id="L235" rel="#L235">235</span>
<span id="L236" rel="#L236">236</span>
<span id="L237" rel="#L237">237</span>
<span id="L238" rel="#L238">238</span>
<span id="L239" rel="#L239">239</span>
<span id="L240" rel="#L240">240</span>
<span id="L241" rel="#L241">241</span>
<span id="L242" rel="#L242">242</span>
<span id="L243" rel="#L243">243</span>
<span id="L244" rel="#L244">244</span>
<span id="L245" rel="#L245">245</span>
<span id="L246" rel="#L246">246</span>
<span id="L247" rel="#L247">247</span>
<span id="L248" rel="#L248">248</span>
<span id="L249" rel="#L249">249</span>
<span id="L250" rel="#L250">250</span>
<span id="L251" rel="#L251">251</span>
<span id="L252" rel="#L252">252</span>
<span id="L253" rel="#L253">253</span>
<span id="L254" rel="#L254">254</span>
<span id="L255" rel="#L255">255</span>
<span id="L256" rel="#L256">256</span>
<span id="L257" rel="#L257">257</span>
<span id="L258" rel="#L258">258</span>
<span id="L259" rel="#L259">259</span>
<span id="L260" rel="#L260">260</span>
<span id="L261" rel="#L261">261</span>
<span id="L262" rel="#L262">262</span>
<span id="L263" rel="#L263">263</span>
<span id="L264" rel="#L264">264</span>
<span id="L265" rel="#L265">265</span>
<span id="L266" rel="#L266">266</span>
<span id="L267" rel="#L267">267</span>
<span id="L268" rel="#L268">268</span>
<span id="L269" rel="#L269">269</span>
<span id="L270" rel="#L270">270</span>
<span id="L271" rel="#L271">271</span>
<span id="L272" rel="#L272">272</span>
<span id="L273" rel="#L273">273</span>
<span id="L274" rel="#L274">274</span>
<span id="L275" rel="#L275">275</span>
<span id="L276" rel="#L276">276</span>
<span id="L277" rel="#L277">277</span>
<span id="L278" rel="#L278">278</span>
<span id="L279" rel="#L279">279</span>
<span id="L280" rel="#L280">280</span>
<span id="L281" rel="#L281">281</span>
<span id="L282" rel="#L282">282</span>
<span id="L283" rel="#L283">283</span>
<span id="L284" rel="#L284">284</span>
<span id="L285" rel="#L285">285</span>
<span id="L286" rel="#L286">286</span>
<span id="L287" rel="#L287">287</span>
<span id="L288" rel="#L288">288</span>
<span id="L289" rel="#L289">289</span>
<span id="L290" rel="#L290">290</span>
<span id="L291" rel="#L291">291</span>
<span id="L292" rel="#L292">292</span>
<span id="L293" rel="#L293">293</span>
<span id="L294" rel="#L294">294</span>
<span id="L295" rel="#L295">295</span>
<span id="L296" rel="#L296">296</span>

          </td>
          <td class="blob-line-code">
                  <div class="highlight"><pre><div class='line' id='LC1'><span class="c">#!/usr/bin/env python</span></div><div class='line' id='LC2'><br/></div><div class='line' id='LC3'><span class="sd">&quot;&quot;&quot;Simple HTTP Server With Upload.</span></div><div class='line' id='LC4'><br/></div><div class='line' id='LC5'><span class="sd">This module builds on BaseHTTPServer by implementing the standard GET</span></div><div class='line' id='LC6'><span class="sd">and HEAD requests in a fairly straightforward manner.</span></div><div class='line' id='LC7'><br/></div><div class='line' id='LC8'><span class="sd">&quot;&quot;&quot;</span></div><div class='line' id='LC9'><br/></div><div class='line' id='LC10'><br/></div><div class='line' id='LC11'><span class="n">__version__</span> <span class="o">=</span> <span class="s">&quot;0.1&quot;</span></div><div class='line' id='LC12'><span class="n">__all__</span> <span class="o">=</span> <span class="p">[</span><span class="s">&quot;SimpleHTTPRequestHandler&quot;</span><span class="p">]</span></div><div class='line' id='LC13'><span class="n">__author__</span> <span class="o">=</span> <span class="s">&quot;bones7456&quot;</span></div><div class='line' id='LC14'><span class="n">__home_page__</span> <span class="o">=</span> <span class="s">&quot;http://li2z.cn/&quot;</span></div><div class='line' id='LC15'><br/></div><div class='line' id='LC16'><span class="kn">import</span> <span class="nn">os</span></div><div class='line' id='LC17'><span class="kn">import</span> <span class="nn">posixpath</span></div><div class='line' id='LC18'><span class="kn">import</span> <span class="nn">BaseHTTPServer</span></div><div class='line' id='LC19'><span class="kn">import</span> <span class="nn">urllib</span></div><div class='line' id='LC20'><span class="kn">import</span> <span class="nn">cgi</span></div><div class='line' id='LC21'><span class="kn">import</span> <span class="nn">shutil</span></div><div class='line' id='LC22'><span class="kn">import</span> <span class="nn">mimetypes</span></div><div class='line' id='LC23'><span class="kn">import</span> <span class="nn">re</span></div><div class='line' id='LC24'><span class="k">try</span><span class="p">:</span></div><div class='line' id='LC25'>&nbsp;&nbsp;&nbsp;&nbsp;<span class="kn">from</span> <span class="nn">cStringIO</span> <span class="kn">import</span> <span class="n">StringIO</span></div><div class='line' id='LC26'><span class="k">except</span> <span class="ne">ImportError</span><span class="p">:</span></div><div class='line' id='LC27'>&nbsp;&nbsp;&nbsp;&nbsp;<span class="kn">from</span> <span class="nn">StringIO</span> <span class="kn">import</span> <span class="n">StringIO</span></div><div class='line' id='LC28'><br/></div><div class='line' id='LC29'><br/></div><div class='line' id='LC30'><span class="k">class</span> <span class="nc">SimpleHTTPRequestHandler</span><span class="p">(</span><span class="n">BaseHTTPServer</span><span class="o">.</span><span class="n">BaseHTTPRequestHandler</span><span class="p">):</span></div><div class='line' id='LC31'><br/></div><div class='line' id='LC32'>&nbsp;&nbsp;&nbsp;&nbsp;<span class="sd">&quot;&quot;&quot;Simple HTTP request handler with GET/HEAD/POST commands.</span></div><div class='line' id='LC33'><br/></div><div class='line' id='LC34'><span class="sd">    This serves files from the current directory and any of its</span></div><div class='line' id='LC35'><span class="sd">    subdirectories.  The MIME type for files is determined by</span></div><div class='line' id='LC36'><span class="sd">    calling the .guess_type() method. And can reveive file uploaded</span></div><div class='line' id='LC37'><span class="sd">    by client.</span></div><div class='line' id='LC38'><br/></div><div class='line' id='LC39'><span class="sd">    The GET/HEAD/POST requests are identical except that the HEAD</span></div><div class='line' id='LC40'><span class="sd">    request omits the actual contents of the file.</span></div><div class='line' id='LC41'><br/></div><div class='line' id='LC42'><span class="sd">    &quot;&quot;&quot;</span></div><div class='line' id='LC43'><br/></div><div class='line' id='LC44'>&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">server_version</span> <span class="o">=</span> <span class="s">&quot;SimpleHTTPWithUpload/&quot;</span> <span class="o">+</span> <span class="n">__version__</span></div><div class='line' id='LC45'><br/></div><div class='line' id='LC46'>&nbsp;&nbsp;&nbsp;&nbsp;<span class="k">def</span> <span class="nf">do_GET</span><span class="p">(</span><span class="bp">self</span><span class="p">):</span></div><div class='line' id='LC47'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="sd">&quot;&quot;&quot;Serve a GET request.&quot;&quot;&quot;</span></div><div class='line' id='LC48'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">f</span> <span class="o">=</span> <span class="bp">self</span><span class="o">.</span><span class="n">send_head</span><span class="p">()</span></div><div class='line' id='LC49'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="k">if</span> <span class="n">f</span><span class="p">:</span></div><div class='line' id='LC50'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="bp">self</span><span class="o">.</span><span class="n">copyfile</span><span class="p">(</span><span class="n">f</span><span class="p">,</span> <span class="bp">self</span><span class="o">.</span><span class="n">wfile</span><span class="p">)</span></div><div class='line' id='LC51'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">f</span><span class="o">.</span><span class="n">close</span><span class="p">()</span></div><div class='line' id='LC52'><br/></div><div class='line' id='LC53'>&nbsp;&nbsp;&nbsp;&nbsp;<span class="k">def</span> <span class="nf">do_HEAD</span><span class="p">(</span><span class="bp">self</span><span class="p">):</span></div><div class='line' id='LC54'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="sd">&quot;&quot;&quot;Serve a HEAD request.&quot;&quot;&quot;</span></div><div class='line' id='LC55'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">f</span> <span class="o">=</span> <span class="bp">self</span><span class="o">.</span><span class="n">send_head</span><span class="p">()</span></div><div class='line' id='LC56'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="k">if</span> <span class="n">f</span><span class="p">:</span></div><div class='line' id='LC57'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">f</span><span class="o">.</span><span class="n">close</span><span class="p">()</span></div><div class='line' id='LC58'><br/></div><div class='line' id='LC59'>&nbsp;&nbsp;&nbsp;&nbsp;<span class="k">def</span> <span class="nf">do_POST</span><span class="p">(</span><span class="bp">self</span><span class="p">):</span></div><div class='line' id='LC60'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="sd">&quot;&quot;&quot;Serve a POST request.&quot;&quot;&quot;</span></div><div class='line' id='LC61'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">r</span><span class="p">,</span> <span class="n">info</span> <span class="o">=</span> <span class="bp">self</span><span class="o">.</span><span class="n">deal_post_data</span><span class="p">()</span></div><div class='line' id='LC62'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="k">print</span> <span class="n">r</span><span class="p">,</span> <span class="n">info</span><span class="p">,</span> <span class="s">&quot;by: &quot;</span><span class="p">,</span> <span class="bp">self</span><span class="o">.</span><span class="n">client_address</span></div><div class='line' id='LC63'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">f</span> <span class="o">=</span> <span class="n">StringIO</span><span class="p">()</span></div><div class='line' id='LC64'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">f</span><span class="o">.</span><span class="n">write</span><span class="p">(</span><span class="s">&#39;&lt;!DOCTYPE html PUBLIC &quot;-//W3C//DTD HTML 3.2 Final//EN&quot;&gt;&#39;</span><span class="p">)</span></div><div class='line' id='LC65'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">f</span><span class="o">.</span><span class="n">write</span><span class="p">(</span><span class="s">&quot;&lt;html&gt;</span><span class="se">\n</span><span class="s">&lt;title&gt;Upload Result Page&lt;/title&gt;</span><span class="se">\n</span><span class="s">&quot;</span><span class="p">)</span></div><div class='line' id='LC66'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">f</span><span class="o">.</span><span class="n">write</span><span class="p">(</span><span class="s">&quot;&lt;body&gt;</span><span class="se">\n</span><span class="s">&lt;h2&gt;Upload Result Page&lt;/h2&gt;</span><span class="se">\n</span><span class="s">&quot;</span><span class="p">)</span></div><div class='line' id='LC67'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">f</span><span class="o">.</span><span class="n">write</span><span class="p">(</span><span class="s">&quot;&lt;hr&gt;</span><span class="se">\n</span><span class="s">&quot;</span><span class="p">)</span></div><div class='line' id='LC68'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="k">if</span> <span class="n">r</span><span class="p">:</span></div><div class='line' id='LC69'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">f</span><span class="o">.</span><span class="n">write</span><span class="p">(</span><span class="s">&quot;&lt;strong&gt;Success:&lt;/strong&gt;&quot;</span><span class="p">)</span></div><div class='line' id='LC70'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="k">else</span><span class="p">:</span></div><div class='line' id='LC71'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">f</span><span class="o">.</span><span class="n">write</span><span class="p">(</span><span class="s">&quot;&lt;strong&gt;Failed:&lt;/strong&gt;&quot;</span><span class="p">)</span></div><div class='line' id='LC72'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">f</span><span class="o">.</span><span class="n">write</span><span class="p">(</span><span class="n">info</span><span class="p">)</span></div><div class='line' id='LC73'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">f</span><span class="o">.</span><span class="n">write</span><span class="p">(</span><span class="s">&quot;&lt;br&gt;&lt;a href=</span><span class="se">\&quot;</span><span class="si">%s</span><span class="se">\&quot;</span><span class="s">&gt;back&lt;/a&gt;&quot;</span> <span class="o">%</span> <span class="bp">self</span><span class="o">.</span><span class="n">headers</span><span class="p">[</span><span class="s">&#39;referer&#39;</span><span class="p">])</span></div><div class='line' id='LC74'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">f</span><span class="o">.</span><span class="n">write</span><span class="p">(</span><span class="s">&quot;&lt;hr&gt;&lt;small&gt;Powered By: bones7456, check new version at &quot;</span><span class="p">)</span></div><div class='line' id='LC75'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">f</span><span class="o">.</span><span class="n">write</span><span class="p">(</span><span class="s">&quot;&lt;a href=</span><span class="se">\&quot;</span><span class="s">http://li2z.cn/?s=SimpleHTTPServerWithUpload</span><span class="se">\&quot;</span><span class="s">&gt;&quot;</span><span class="p">)</span></div><div class='line' id='LC76'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">f</span><span class="o">.</span><span class="n">write</span><span class="p">(</span><span class="s">&quot;here&lt;/a&gt;.&lt;/small&gt;&lt;/body&gt;</span><span class="se">\n</span><span class="s">&lt;/html&gt;</span><span class="se">\n</span><span class="s">&quot;</span><span class="p">)</span></div><div class='line' id='LC77'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">length</span> <span class="o">=</span> <span class="n">f</span><span class="o">.</span><span class="n">tell</span><span class="p">()</span></div><div class='line' id='LC78'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">f</span><span class="o">.</span><span class="n">seek</span><span class="p">(</span><span class="mi">0</span><span class="p">)</span></div><div class='line' id='LC79'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="bp">self</span><span class="o">.</span><span class="n">send_response</span><span class="p">(</span><span class="mi">200</span><span class="p">)</span></div><div class='line' id='LC80'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="bp">self</span><span class="o">.</span><span class="n">send_header</span><span class="p">(</span><span class="s">&quot;Content-type&quot;</span><span class="p">,</span> <span class="s">&quot;text/html&quot;</span><span class="p">)</span></div><div class='line' id='LC81'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="bp">self</span><span class="o">.</span><span class="n">send_header</span><span class="p">(</span><span class="s">&quot;Content-Length&quot;</span><span class="p">,</span> <span class="nb">str</span><span class="p">(</span><span class="n">length</span><span class="p">))</span></div><div class='line' id='LC82'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="bp">self</span><span class="o">.</span><span class="n">end_headers</span><span class="p">()</span></div><div class='line' id='LC83'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="k">if</span> <span class="n">f</span><span class="p">:</span></div><div class='line' id='LC84'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="bp">self</span><span class="o">.</span><span class="n">copyfile</span><span class="p">(</span><span class="n">f</span><span class="p">,</span> <span class="bp">self</span><span class="o">.</span><span class="n">wfile</span><span class="p">)</span></div><div class='line' id='LC85'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">f</span><span class="o">.</span><span class="n">close</span><span class="p">()</span></div><div class='line' id='LC86'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;</div><div class='line' id='LC87'>&nbsp;&nbsp;&nbsp;&nbsp;<span class="k">def</span> <span class="nf">deal_post_data</span><span class="p">(</span><span class="bp">self</span><span class="p">):</span></div><div class='line' id='LC88'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">boundary</span> <span class="o">=</span> <span class="bp">self</span><span class="o">.</span><span class="n">headers</span><span class="o">.</span><span class="n">plisttext</span><span class="o">.</span><span class="n">split</span><span class="p">(</span><span class="s">&quot;=&quot;</span><span class="p">)[</span><span class="mi">1</span><span class="p">]</span></div><div class='line' id='LC89'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">remainbytes</span> <span class="o">=</span> <span class="nb">int</span><span class="p">(</span><span class="bp">self</span><span class="o">.</span><span class="n">headers</span><span class="p">[</span><span class="s">&#39;content-length&#39;</span><span class="p">])</span></div><div class='line' id='LC90'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">line</span> <span class="o">=</span> <span class="bp">self</span><span class="o">.</span><span class="n">rfile</span><span class="o">.</span><span class="n">readline</span><span class="p">()</span></div><div class='line' id='LC91'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">remainbytes</span> <span class="o">-=</span> <span class="nb">len</span><span class="p">(</span><span class="n">line</span><span class="p">)</span></div><div class='line' id='LC92'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="k">if</span> <span class="ow">not</span> <span class="n">boundary</span> <span class="ow">in</span> <span class="n">line</span><span class="p">:</span></div><div class='line' id='LC93'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="k">return</span> <span class="p">(</span><span class="bp">False</span><span class="p">,</span> <span class="s">&quot;Content NOT begin with boundary&quot;</span><span class="p">)</span></div><div class='line' id='LC94'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">line</span> <span class="o">=</span> <span class="bp">self</span><span class="o">.</span><span class="n">rfile</span><span class="o">.</span><span class="n">readline</span><span class="p">()</span></div><div class='line' id='LC95'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">remainbytes</span> <span class="o">-=</span> <span class="nb">len</span><span class="p">(</span><span class="n">line</span><span class="p">)</span></div><div class='line' id='LC96'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">fn</span> <span class="o">=</span> <span class="n">re</span><span class="o">.</span><span class="n">findall</span><span class="p">(</span><span class="s">r&#39;Content-Disposition.*name=&quot;file&quot;; filename=&quot;(.*)&quot;&#39;</span><span class="p">,</span> <span class="n">line</span><span class="p">)</span></div><div class='line' id='LC97'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="k">if</span> <span class="ow">not</span> <span class="n">fn</span><span class="p">:</span></div><div class='line' id='LC98'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="k">return</span> <span class="p">(</span><span class="bp">False</span><span class="p">,</span> <span class="s">&quot;Can&#39;t find out file name...&quot;</span><span class="p">)</span></div><div class='line' id='LC99'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">path</span> <span class="o">=</span> <span class="bp">self</span><span class="o">.</span><span class="n">translate_path</span><span class="p">(</span><span class="bp">self</span><span class="o">.</span><span class="n">path</span><span class="p">)</span></div><div class='line' id='LC100'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">fn</span> <span class="o">=</span> <span class="n">os</span><span class="o">.</span><span class="n">path</span><span class="o">.</span><span class="n">join</span><span class="p">(</span><span class="n">path</span><span class="p">,</span> <span class="n">fn</span><span class="p">[</span><span class="mi">0</span><span class="p">])</span></div><div class='line' id='LC101'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="k">while</span> <span class="n">os</span><span class="o">.</span><span class="n">path</span><span class="o">.</span><span class="n">exists</span><span class="p">(</span><span class="n">fn</span><span class="p">):</span></div><div class='line' id='LC102'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">fn</span> <span class="o">+=</span> <span class="s">&quot;_&quot;</span></div><div class='line' id='LC103'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">line</span> <span class="o">=</span> <span class="bp">self</span><span class="o">.</span><span class="n">rfile</span><span class="o">.</span><span class="n">readline</span><span class="p">()</span></div><div class='line' id='LC104'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">remainbytes</span> <span class="o">-=</span> <span class="nb">len</span><span class="p">(</span><span class="n">line</span><span class="p">)</span></div><div class='line' id='LC105'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">line</span> <span class="o">=</span> <span class="bp">self</span><span class="o">.</span><span class="n">rfile</span><span class="o">.</span><span class="n">readline</span><span class="p">()</span></div><div class='line' id='LC106'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">remainbytes</span> <span class="o">-=</span> <span class="nb">len</span><span class="p">(</span><span class="n">line</span><span class="p">)</span></div><div class='line' id='LC107'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="k">try</span><span class="p">:</span></div><div class='line' id='LC108'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">out</span> <span class="o">=</span> <span class="nb">open</span><span class="p">(</span><span class="n">fn</span><span class="p">,</span> <span class="s">&#39;wb&#39;</span><span class="p">)</span></div><div class='line' id='LC109'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="k">except</span> <span class="ne">IOError</span><span class="p">:</span></div><div class='line' id='LC110'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="k">return</span> <span class="p">(</span><span class="bp">False</span><span class="p">,</span> <span class="s">&quot;Can&#39;t create file to write, do you have permission to write?&quot;</span><span class="p">)</span></div><div class='line' id='LC111'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;</div><div class='line' id='LC112'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">preline</span> <span class="o">=</span> <span class="bp">self</span><span class="o">.</span><span class="n">rfile</span><span class="o">.</span><span class="n">readline</span><span class="p">()</span></div><div class='line' id='LC113'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">remainbytes</span> <span class="o">-=</span> <span class="nb">len</span><span class="p">(</span><span class="n">preline</span><span class="p">)</span></div><div class='line' id='LC114'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="k">while</span> <span class="n">remainbytes</span> <span class="o">&gt;</span> <span class="mi">0</span><span class="p">:</span></div><div class='line' id='LC115'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">line</span> <span class="o">=</span> <span class="bp">self</span><span class="o">.</span><span class="n">rfile</span><span class="o">.</span><span class="n">readline</span><span class="p">()</span></div><div class='line' id='LC116'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">remainbytes</span> <span class="o">-=</span> <span class="nb">len</span><span class="p">(</span><span class="n">line</span><span class="p">)</span></div><div class='line' id='LC117'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="k">if</span> <span class="n">boundary</span> <span class="ow">in</span> <span class="n">line</span><span class="p">:</span></div><div class='line' id='LC118'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">preline</span> <span class="o">=</span> <span class="n">preline</span><span class="p">[</span><span class="mi">0</span><span class="p">:</span><span class="o">-</span><span class="mi">1</span><span class="p">]</span></div><div class='line' id='LC119'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="k">if</span> <span class="n">preline</span><span class="o">.</span><span class="n">endswith</span><span class="p">(</span><span class="s">&#39;</span><span class="se">\r</span><span class="s">&#39;</span><span class="p">):</span></div><div class='line' id='LC120'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">preline</span> <span class="o">=</span> <span class="n">preline</span><span class="p">[</span><span class="mi">0</span><span class="p">:</span><span class="o">-</span><span class="mi">1</span><span class="p">]</span></div><div class='line' id='LC121'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">out</span><span class="o">.</span><span class="n">write</span><span class="p">(</span><span class="n">preline</span><span class="p">)</span></div><div class='line' id='LC122'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">out</span><span class="o">.</span><span class="n">close</span><span class="p">()</span></div><div class='line' id='LC123'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="k">return</span> <span class="p">(</span><span class="bp">True</span><span class="p">,</span> <span class="s">&quot;File &#39;</span><span class="si">%s</span><span class="s">&#39; upload success!&quot;</span> <span class="o">%</span> <span class="n">fn</span><span class="p">)</span></div><div class='line' id='LC124'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="k">else</span><span class="p">:</span></div><div class='line' id='LC125'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">out</span><span class="o">.</span><span class="n">write</span><span class="p">(</span><span class="n">preline</span><span class="p">)</span></div><div class='line' id='LC126'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">preline</span> <span class="o">=</span> <span class="n">line</span></div><div class='line' id='LC127'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="k">return</span> <span class="p">(</span><span class="bp">False</span><span class="p">,</span> <span class="s">&quot;Unexpect Ends of data.&quot;</span><span class="p">)</span></div><div class='line' id='LC128'><br/></div><div class='line' id='LC129'>&nbsp;&nbsp;&nbsp;&nbsp;<span class="k">def</span> <span class="nf">send_head</span><span class="p">(</span><span class="bp">self</span><span class="p">):</span></div><div class='line' id='LC130'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="sd">&quot;&quot;&quot;Common code for GET and HEAD commands.</span></div><div class='line' id='LC131'><br/></div><div class='line' id='LC132'><span class="sd">        This sends the response code and MIME headers.</span></div><div class='line' id='LC133'><br/></div><div class='line' id='LC134'><span class="sd">        Return value is either a file object (which has to be copied</span></div><div class='line' id='LC135'><span class="sd">        to the outputfile by the caller unless the command was HEAD,</span></div><div class='line' id='LC136'><span class="sd">        and must be closed by the caller under all circumstances), or</span></div><div class='line' id='LC137'><span class="sd">        None, in which case the caller has nothing further to do.</span></div><div class='line' id='LC138'><br/></div><div class='line' id='LC139'><span class="sd">        &quot;&quot;&quot;</span></div><div class='line' id='LC140'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">path</span> <span class="o">=</span> <span class="bp">self</span><span class="o">.</span><span class="n">translate_path</span><span class="p">(</span><span class="bp">self</span><span class="o">.</span><span class="n">path</span><span class="p">)</span></div><div class='line' id='LC141'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">f</span> <span class="o">=</span> <span class="bp">None</span></div><div class='line' id='LC142'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="k">if</span> <span class="n">os</span><span class="o">.</span><span class="n">path</span><span class="o">.</span><span class="n">isdir</span><span class="p">(</span><span class="n">path</span><span class="p">):</span></div><div class='line' id='LC143'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="k">if</span> <span class="ow">not</span> <span class="bp">self</span><span class="o">.</span><span class="n">path</span><span class="o">.</span><span class="n">endswith</span><span class="p">(</span><span class="s">&#39;/&#39;</span><span class="p">):</span></div><div class='line' id='LC144'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="c"># redirect browser - doing basically what apache does</span></div><div class='line' id='LC145'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="bp">self</span><span class="o">.</span><span class="n">send_response</span><span class="p">(</span><span class="mi">301</span><span class="p">)</span></div><div class='line' id='LC146'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="bp">self</span><span class="o">.</span><span class="n">send_header</span><span class="p">(</span><span class="s">&quot;Location&quot;</span><span class="p">,</span> <span class="bp">self</span><span class="o">.</span><span class="n">path</span> <span class="o">+</span> <span class="s">&quot;/&quot;</span><span class="p">)</span></div><div class='line' id='LC147'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="bp">self</span><span class="o">.</span><span class="n">end_headers</span><span class="p">()</span></div><div class='line' id='LC148'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="k">return</span> <span class="bp">None</span></div><div class='line' id='LC149'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="k">for</span> <span class="n">index</span> <span class="ow">in</span> <span class="s">&quot;index.html&quot;</span><span class="p">,</span> <span class="s">&quot;index.htm&quot;</span><span class="p">:</span></div><div class='line' id='LC150'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">index</span> <span class="o">=</span> <span class="n">os</span><span class="o">.</span><span class="n">path</span><span class="o">.</span><span class="n">join</span><span class="p">(</span><span class="n">path</span><span class="p">,</span> <span class="n">index</span><span class="p">)</span></div><div class='line' id='LC151'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="k">if</span> <span class="n">os</span><span class="o">.</span><span class="n">path</span><span class="o">.</span><span class="n">exists</span><span class="p">(</span><span class="n">index</span><span class="p">):</span></div><div class='line' id='LC152'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">path</span> <span class="o">=</span> <span class="n">index</span></div><div class='line' id='LC153'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="k">break</span></div><div class='line' id='LC154'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="k">else</span><span class="p">:</span></div><div class='line' id='LC155'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="k">return</span> <span class="bp">self</span><span class="o">.</span><span class="n">list_directory</span><span class="p">(</span><span class="n">path</span><span class="p">)</span></div><div class='line' id='LC156'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">ctype</span> <span class="o">=</span> <span class="bp">self</span><span class="o">.</span><span class="n">guess_type</span><span class="p">(</span><span class="n">path</span><span class="p">)</span></div><div class='line' id='LC157'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="k">try</span><span class="p">:</span></div><div class='line' id='LC158'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="c"># Always read in binary mode. Opening files in text mode may cause</span></div><div class='line' id='LC159'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="c"># newline translations, making the actual size of the content</span></div><div class='line' id='LC160'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="c"># transmitted *less* than the content-length!</span></div><div class='line' id='LC161'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">f</span> <span class="o">=</span> <span class="nb">open</span><span class="p">(</span><span class="n">path</span><span class="p">,</span> <span class="s">&#39;rb&#39;</span><span class="p">)</span></div><div class='line' id='LC162'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="k">except</span> <span class="ne">IOError</span><span class="p">:</span></div><div class='line' id='LC163'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="bp">self</span><span class="o">.</span><span class="n">send_error</span><span class="p">(</span><span class="mi">404</span><span class="p">,</span> <span class="s">&quot;File not found&quot;</span><span class="p">)</span></div><div class='line' id='LC164'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="k">return</span> <span class="bp">None</span></div><div class='line' id='LC165'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="bp">self</span><span class="o">.</span><span class="n">send_response</span><span class="p">(</span><span class="mi">200</span><span class="p">)</span></div><div class='line' id='LC166'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="bp">self</span><span class="o">.</span><span class="n">send_header</span><span class="p">(</span><span class="s">&quot;Content-type&quot;</span><span class="p">,</span> <span class="n">ctype</span><span class="p">)</span></div><div class='line' id='LC167'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">fs</span> <span class="o">=</span> <span class="n">os</span><span class="o">.</span><span class="n">fstat</span><span class="p">(</span><span class="n">f</span><span class="o">.</span><span class="n">fileno</span><span class="p">())</span></div><div class='line' id='LC168'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="bp">self</span><span class="o">.</span><span class="n">send_header</span><span class="p">(</span><span class="s">&quot;Content-Length&quot;</span><span class="p">,</span> <span class="nb">str</span><span class="p">(</span><span class="n">fs</span><span class="p">[</span><span class="mi">6</span><span class="p">]))</span></div><div class='line' id='LC169'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="bp">self</span><span class="o">.</span><span class="n">send_header</span><span class="p">(</span><span class="s">&quot;Last-Modified&quot;</span><span class="p">,</span> <span class="bp">self</span><span class="o">.</span><span class="n">date_time_string</span><span class="p">(</span><span class="n">fs</span><span class="o">.</span><span class="n">st_mtime</span><span class="p">))</span></div><div class='line' id='LC170'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="bp">self</span><span class="o">.</span><span class="n">end_headers</span><span class="p">()</span></div><div class='line' id='LC171'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="k">return</span> <span class="n">f</span></div><div class='line' id='LC172'><br/></div><div class='line' id='LC173'>&nbsp;&nbsp;&nbsp;&nbsp;<span class="k">def</span> <span class="nf">list_directory</span><span class="p">(</span><span class="bp">self</span><span class="p">,</span> <span class="n">path</span><span class="p">):</span></div><div class='line' id='LC174'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="sd">&quot;&quot;&quot;Helper to produce a directory listing (absent index.html).</span></div><div class='line' id='LC175'><br/></div><div class='line' id='LC176'><span class="sd">        Return value is either a file object, or None (indicating an</span></div><div class='line' id='LC177'><span class="sd">        error).  In either case, the headers are sent, making the</span></div><div class='line' id='LC178'><span class="sd">        interface the same as for send_head().</span></div><div class='line' id='LC179'><br/></div><div class='line' id='LC180'><span class="sd">        &quot;&quot;&quot;</span></div><div class='line' id='LC181'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="k">try</span><span class="p">:</span></div><div class='line' id='LC182'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="nb">list</span> <span class="o">=</span> <span class="n">os</span><span class="o">.</span><span class="n">listdir</span><span class="p">(</span><span class="n">path</span><span class="p">)</span></div><div class='line' id='LC183'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="k">except</span> <span class="n">os</span><span class="o">.</span><span class="n">error</span><span class="p">:</span></div><div class='line' id='LC184'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="bp">self</span><span class="o">.</span><span class="n">send_error</span><span class="p">(</span><span class="mi">404</span><span class="p">,</span> <span class="s">&quot;No permission to list directory&quot;</span><span class="p">)</span></div><div class='line' id='LC185'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="k">return</span> <span class="bp">None</span></div><div class='line' id='LC186'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="nb">list</span><span class="o">.</span><span class="n">sort</span><span class="p">(</span><span class="n">key</span><span class="o">=</span><span class="k">lambda</span> <span class="n">a</span><span class="p">:</span> <span class="n">a</span><span class="o">.</span><span class="n">lower</span><span class="p">())</span></div><div class='line' id='LC187'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">f</span> <span class="o">=</span> <span class="n">StringIO</span><span class="p">()</span></div><div class='line' id='LC188'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">displaypath</span> <span class="o">=</span> <span class="n">cgi</span><span class="o">.</span><span class="n">escape</span><span class="p">(</span><span class="n">urllib</span><span class="o">.</span><span class="n">unquote</span><span class="p">(</span><span class="bp">self</span><span class="o">.</span><span class="n">path</span><span class="p">))</span></div><div class='line' id='LC189'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">f</span><span class="o">.</span><span class="n">write</span><span class="p">(</span><span class="s">&#39;&lt;!DOCTYPE html PUBLIC &quot;-//W3C//DTD HTML 3.2 Final//EN&quot;&gt;&#39;</span><span class="p">)</span></div><div class='line' id='LC190'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">f</span><span class="o">.</span><span class="n">write</span><span class="p">(</span><span class="s">&quot;&lt;html&gt;</span><span class="se">\n</span><span class="s">&lt;title&gt;Directory listing for </span><span class="si">%s</span><span class="s">&lt;/title&gt;</span><span class="se">\n</span><span class="s">&quot;</span> <span class="o">%</span> <span class="n">displaypath</span><span class="p">)</span></div><div class='line' id='LC191'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">f</span><span class="o">.</span><span class="n">write</span><span class="p">(</span><span class="s">&quot;&lt;body&gt;</span><span class="se">\n</span><span class="s">&lt;h2&gt;Directory listing for </span><span class="si">%s</span><span class="s">&lt;/h2&gt;</span><span class="se">\n</span><span class="s">&quot;</span> <span class="o">%</span> <span class="n">displaypath</span><span class="p">)</span></div><div class='line' id='LC192'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">f</span><span class="o">.</span><span class="n">write</span><span class="p">(</span><span class="s">&quot;&lt;hr&gt;</span><span class="se">\n</span><span class="s">&quot;</span><span class="p">)</span></div><div class='line' id='LC193'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">f</span><span class="o">.</span><span class="n">write</span><span class="p">(</span><span class="s">&quot;&lt;form ENCTYPE=</span><span class="se">\&quot;</span><span class="s">multipart/form-data</span><span class="se">\&quot;</span><span class="s"> method=</span><span class="se">\&quot;</span><span class="s">post</span><span class="se">\&quot;</span><span class="s">&gt;&quot;</span><span class="p">)</span></div><div class='line' id='LC194'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">f</span><span class="o">.</span><span class="n">write</span><span class="p">(</span><span class="s">&quot;&lt;input name=</span><span class="se">\&quot;</span><span class="s">file</span><span class="se">\&quot;</span><span class="s"> type=</span><span class="se">\&quot;</span><span class="s">file</span><span class="se">\&quot;</span><span class="s">/&gt;&quot;</span><span class="p">)</span></div><div class='line' id='LC195'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">f</span><span class="o">.</span><span class="n">write</span><span class="p">(</span><span class="s">&quot;&lt;input type=</span><span class="se">\&quot;</span><span class="s">submit</span><span class="se">\&quot;</span><span class="s"> value=</span><span class="se">\&quot;</span><span class="s">upload</span><span class="se">\&quot;</span><span class="s">/&gt;&lt;/form&gt;</span><span class="se">\n</span><span class="s">&quot;</span><span class="p">)</span></div><div class='line' id='LC196'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">f</span><span class="o">.</span><span class="n">write</span><span class="p">(</span><span class="s">&quot;&lt;hr&gt;</span><span class="se">\n</span><span class="s">&lt;ul&gt;</span><span class="se">\n</span><span class="s">&quot;</span><span class="p">)</span></div><div class='line' id='LC197'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="k">for</span> <span class="n">name</span> <span class="ow">in</span> <span class="nb">list</span><span class="p">:</span></div><div class='line' id='LC198'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">fullname</span> <span class="o">=</span> <span class="n">os</span><span class="o">.</span><span class="n">path</span><span class="o">.</span><span class="n">join</span><span class="p">(</span><span class="n">path</span><span class="p">,</span> <span class="n">name</span><span class="p">)</span></div><div class='line' id='LC199'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">displayname</span> <span class="o">=</span> <span class="n">linkname</span> <span class="o">=</span> <span class="n">name</span></div><div class='line' id='LC200'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="c"># Append / for directories or @ for symbolic links</span></div><div class='line' id='LC201'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="k">if</span> <span class="n">os</span><span class="o">.</span><span class="n">path</span><span class="o">.</span><span class="n">isdir</span><span class="p">(</span><span class="n">fullname</span><span class="p">):</span></div><div class='line' id='LC202'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">displayname</span> <span class="o">=</span> <span class="n">name</span> <span class="o">+</span> <span class="s">&quot;/&quot;</span></div><div class='line' id='LC203'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">linkname</span> <span class="o">=</span> <span class="n">name</span> <span class="o">+</span> <span class="s">&quot;/&quot;</span></div><div class='line' id='LC204'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="k">if</span> <span class="n">os</span><span class="o">.</span><span class="n">path</span><span class="o">.</span><span class="n">islink</span><span class="p">(</span><span class="n">fullname</span><span class="p">):</span></div><div class='line' id='LC205'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">displayname</span> <span class="o">=</span> <span class="n">name</span> <span class="o">+</span> <span class="s">&quot;@&quot;</span></div><div class='line' id='LC206'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="c"># Note: a link to a directory displays with @ and links with /</span></div><div class='line' id='LC207'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">f</span><span class="o">.</span><span class="n">write</span><span class="p">(</span><span class="s">&#39;&lt;li&gt;&lt;a href=&quot;</span><span class="si">%s</span><span class="s">&quot;&gt;</span><span class="si">%s</span><span class="s">&lt;/a&gt;</span><span class="se">\n</span><span class="s">&#39;</span></div><div class='line' id='LC208'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="o">%</span> <span class="p">(</span><span class="n">urllib</span><span class="o">.</span><span class="n">quote</span><span class="p">(</span><span class="n">linkname</span><span class="p">),</span> <span class="n">cgi</span><span class="o">.</span><span class="n">escape</span><span class="p">(</span><span class="n">displayname</span><span class="p">)))</span></div><div class='line' id='LC209'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">f</span><span class="o">.</span><span class="n">write</span><span class="p">(</span><span class="s">&quot;&lt;/ul&gt;</span><span class="se">\n</span><span class="s">&lt;hr&gt;</span><span class="se">\n</span><span class="s">&lt;/body&gt;</span><span class="se">\n</span><span class="s">&lt;/html&gt;</span><span class="se">\n</span><span class="s">&quot;</span><span class="p">)</span></div><div class='line' id='LC210'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">length</span> <span class="o">=</span> <span class="n">f</span><span class="o">.</span><span class="n">tell</span><span class="p">()</span></div><div class='line' id='LC211'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">f</span><span class="o">.</span><span class="n">seek</span><span class="p">(</span><span class="mi">0</span><span class="p">)</span></div><div class='line' id='LC212'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="bp">self</span><span class="o">.</span><span class="n">send_response</span><span class="p">(</span><span class="mi">200</span><span class="p">)</span></div><div class='line' id='LC213'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="bp">self</span><span class="o">.</span><span class="n">send_header</span><span class="p">(</span><span class="s">&quot;Content-type&quot;</span><span class="p">,</span> <span class="s">&quot;text/html&quot;</span><span class="p">)</span></div><div class='line' id='LC214'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="bp">self</span><span class="o">.</span><span class="n">send_header</span><span class="p">(</span><span class="s">&quot;Content-Length&quot;</span><span class="p">,</span> <span class="nb">str</span><span class="p">(</span><span class="n">length</span><span class="p">))</span></div><div class='line' id='LC215'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="bp">self</span><span class="o">.</span><span class="n">end_headers</span><span class="p">()</span></div><div class='line' id='LC216'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="k">return</span> <span class="n">f</span></div><div class='line' id='LC217'><br/></div><div class='line' id='LC218'>&nbsp;&nbsp;&nbsp;&nbsp;<span class="k">def</span> <span class="nf">translate_path</span><span class="p">(</span><span class="bp">self</span><span class="p">,</span> <span class="n">path</span><span class="p">):</span></div><div class='line' id='LC219'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="sd">&quot;&quot;&quot;Translate a /-separated PATH to the local filename syntax.</span></div><div class='line' id='LC220'><br/></div><div class='line' id='LC221'><span class="sd">        Components that mean special things to the local file system</span></div><div class='line' id='LC222'><span class="sd">        (e.g. drive or directory names) are ignored.  (XXX They should</span></div><div class='line' id='LC223'><span class="sd">        probably be diagnosed.)</span></div><div class='line' id='LC224'><br/></div><div class='line' id='LC225'><span class="sd">        &quot;&quot;&quot;</span></div><div class='line' id='LC226'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="c"># abandon query parameters</span></div><div class='line' id='LC227'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">path</span> <span class="o">=</span> <span class="n">path</span><span class="o">.</span><span class="n">split</span><span class="p">(</span><span class="s">&#39;?&#39;</span><span class="p">,</span><span class="mi">1</span><span class="p">)[</span><span class="mi">0</span><span class="p">]</span></div><div class='line' id='LC228'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">path</span> <span class="o">=</span> <span class="n">path</span><span class="o">.</span><span class="n">split</span><span class="p">(</span><span class="s">&#39;#&#39;</span><span class="p">,</span><span class="mi">1</span><span class="p">)[</span><span class="mi">0</span><span class="p">]</span></div><div class='line' id='LC229'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">path</span> <span class="o">=</span> <span class="n">posixpath</span><span class="o">.</span><span class="n">normpath</span><span class="p">(</span><span class="n">urllib</span><span class="o">.</span><span class="n">unquote</span><span class="p">(</span><span class="n">path</span><span class="p">))</span></div><div class='line' id='LC230'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">words</span> <span class="o">=</span> <span class="n">path</span><span class="o">.</span><span class="n">split</span><span class="p">(</span><span class="s">&#39;/&#39;</span><span class="p">)</span></div><div class='line' id='LC231'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">words</span> <span class="o">=</span> <span class="nb">filter</span><span class="p">(</span><span class="bp">None</span><span class="p">,</span> <span class="n">words</span><span class="p">)</span></div><div class='line' id='LC232'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">path</span> <span class="o">=</span> <span class="n">os</span><span class="o">.</span><span class="n">getcwd</span><span class="p">()</span></div><div class='line' id='LC233'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="k">for</span> <span class="n">word</span> <span class="ow">in</span> <span class="n">words</span><span class="p">:</span></div><div class='line' id='LC234'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">drive</span><span class="p">,</span> <span class="n">word</span> <span class="o">=</span> <span class="n">os</span><span class="o">.</span><span class="n">path</span><span class="o">.</span><span class="n">splitdrive</span><span class="p">(</span><span class="n">word</span><span class="p">)</span></div><div class='line' id='LC235'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">head</span><span class="p">,</span> <span class="n">word</span> <span class="o">=</span> <span class="n">os</span><span class="o">.</span><span class="n">path</span><span class="o">.</span><span class="n">split</span><span class="p">(</span><span class="n">word</span><span class="p">)</span></div><div class='line' id='LC236'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="k">if</span> <span class="n">word</span> <span class="ow">in</span> <span class="p">(</span><span class="n">os</span><span class="o">.</span><span class="n">curdir</span><span class="p">,</span> <span class="n">os</span><span class="o">.</span><span class="n">pardir</span><span class="p">):</span> <span class="k">continue</span></div><div class='line' id='LC237'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">path</span> <span class="o">=</span> <span class="n">os</span><span class="o">.</span><span class="n">path</span><span class="o">.</span><span class="n">join</span><span class="p">(</span><span class="n">path</span><span class="p">,</span> <span class="n">word</span><span class="p">)</span></div><div class='line' id='LC238'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="k">return</span> <span class="n">path</span></div><div class='line' id='LC239'><br/></div><div class='line' id='LC240'>&nbsp;&nbsp;&nbsp;&nbsp;<span class="k">def</span> <span class="nf">copyfile</span><span class="p">(</span><span class="bp">self</span><span class="p">,</span> <span class="n">source</span><span class="p">,</span> <span class="n">outputfile</span><span class="p">):</span></div><div class='line' id='LC241'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="sd">&quot;&quot;&quot;Copy all data between two file objects.</span></div><div class='line' id='LC242'><br/></div><div class='line' id='LC243'><span class="sd">        The SOURCE argument is a file object open for reading</span></div><div class='line' id='LC244'><span class="sd">        (or anything with a read() method) and the DESTINATION</span></div><div class='line' id='LC245'><span class="sd">        argument is a file object open for writing (or</span></div><div class='line' id='LC246'><span class="sd">        anything with a write() method).</span></div><div class='line' id='LC247'><br/></div><div class='line' id='LC248'><span class="sd">        The only reason for overriding this would be to change</span></div><div class='line' id='LC249'><span class="sd">        the block size or perhaps to replace newlines by CRLF</span></div><div class='line' id='LC250'><span class="sd">        -- note however that this the default server uses this</span></div><div class='line' id='LC251'><span class="sd">        to copy binary data as well.</span></div><div class='line' id='LC252'><br/></div><div class='line' id='LC253'><span class="sd">        &quot;&quot;&quot;</span></div><div class='line' id='LC254'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">shutil</span><span class="o">.</span><span class="n">copyfileobj</span><span class="p">(</span><span class="n">source</span><span class="p">,</span> <span class="n">outputfile</span><span class="p">)</span></div><div class='line' id='LC255'><br/></div><div class='line' id='LC256'>&nbsp;&nbsp;&nbsp;&nbsp;<span class="k">def</span> <span class="nf">guess_type</span><span class="p">(</span><span class="bp">self</span><span class="p">,</span> <span class="n">path</span><span class="p">):</span></div><div class='line' id='LC257'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="sd">&quot;&quot;&quot;Guess the type of a file.</span></div><div class='line' id='LC258'><br/></div><div class='line' id='LC259'><span class="sd">        Argument is a PATH (a filename).</span></div><div class='line' id='LC260'><br/></div><div class='line' id='LC261'><span class="sd">        Return value is a string of the form type/subtype,</span></div><div class='line' id='LC262'><span class="sd">        usable for a MIME Content-type header.</span></div><div class='line' id='LC263'><br/></div><div class='line' id='LC264'><span class="sd">        The default implementation looks the file&#39;s extension</span></div><div class='line' id='LC265'><span class="sd">        up in the table self.extensions_map, using application/octet-stream</span></div><div class='line' id='LC266'><span class="sd">        as a default; however it would be permissible (if</span></div><div class='line' id='LC267'><span class="sd">        slow) to look inside the data to make a better guess.</span></div><div class='line' id='LC268'><br/></div><div class='line' id='LC269'><span class="sd">        &quot;&quot;&quot;</span></div><div class='line' id='LC270'><br/></div><div class='line' id='LC271'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">base</span><span class="p">,</span> <span class="n">ext</span> <span class="o">=</span> <span class="n">posixpath</span><span class="o">.</span><span class="n">splitext</span><span class="p">(</span><span class="n">path</span><span class="p">)</span></div><div class='line' id='LC272'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="k">if</span> <span class="n">ext</span> <span class="ow">in</span> <span class="bp">self</span><span class="o">.</span><span class="n">extensions_map</span><span class="p">:</span></div><div class='line' id='LC273'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="k">return</span> <span class="bp">self</span><span class="o">.</span><span class="n">extensions_map</span><span class="p">[</span><span class="n">ext</span><span class="p">]</span></div><div class='line' id='LC274'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">ext</span> <span class="o">=</span> <span class="n">ext</span><span class="o">.</span><span class="n">lower</span><span class="p">()</span></div><div class='line' id='LC275'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="k">if</span> <span class="n">ext</span> <span class="ow">in</span> <span class="bp">self</span><span class="o">.</span><span class="n">extensions_map</span><span class="p">:</span></div><div class='line' id='LC276'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="k">return</span> <span class="bp">self</span><span class="o">.</span><span class="n">extensions_map</span><span class="p">[</span><span class="n">ext</span><span class="p">]</span></div><div class='line' id='LC277'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="k">else</span><span class="p">:</span></div><div class='line' id='LC278'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="k">return</span> <span class="bp">self</span><span class="o">.</span><span class="n">extensions_map</span><span class="p">[</span><span class="s">&#39;&#39;</span><span class="p">]</span></div><div class='line' id='LC279'><br/></div><div class='line' id='LC280'>&nbsp;&nbsp;&nbsp;&nbsp;<span class="k">if</span> <span class="ow">not</span> <span class="n">mimetypes</span><span class="o">.</span><span class="n">inited</span><span class="p">:</span></div><div class='line' id='LC281'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">mimetypes</span><span class="o">.</span><span class="n">init</span><span class="p">()</span> <span class="c"># try to read system mime.types</span></div><div class='line' id='LC282'>&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">extensions_map</span> <span class="o">=</span> <span class="n">mimetypes</span><span class="o">.</span><span class="n">types_map</span><span class="o">.</span><span class="n">copy</span><span class="p">()</span></div><div class='line' id='LC283'>&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">extensions_map</span><span class="o">.</span><span class="n">update</span><span class="p">({</span></div><div class='line' id='LC284'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="s">&#39;&#39;</span><span class="p">:</span> <span class="s">&#39;application/octet-stream&#39;</span><span class="p">,</span> <span class="c"># Default</span></div><div class='line' id='LC285'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="s">&#39;.py&#39;</span><span class="p">:</span> <span class="s">&#39;text/plain&#39;</span><span class="p">,</span></div><div class='line' id='LC286'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="s">&#39;.c&#39;</span><span class="p">:</span> <span class="s">&#39;text/plain&#39;</span><span class="p">,</span></div><div class='line' id='LC287'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="s">&#39;.h&#39;</span><span class="p">:</span> <span class="s">&#39;text/plain&#39;</span><span class="p">,</span></div><div class='line' id='LC288'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="p">})</span></div><div class='line' id='LC289'><br/></div><div class='line' id='LC290'><br/></div><div class='line' id='LC291'><span class="k">def</span> <span class="nf">test</span><span class="p">(</span><span class="n">HandlerClass</span> <span class="o">=</span> <span class="n">SimpleHTTPRequestHandler</span><span class="p">,</span></div><div class='line' id='LC292'>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">ServerClass</span> <span class="o">=</span> <span class="n">BaseHTTPServer</span><span class="o">.</span><span class="n">HTTPServer</span><span class="p">):</span></div><div class='line' id='LC293'>&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">BaseHTTPServer</span><span class="o">.</span><span class="n">test</span><span class="p">(</span><span class="n">HandlerClass</span><span class="p">,</span> <span class="n">ServerClass</span><span class="p">)</span></div><div class='line' id='LC294'><br/></div><div class='line' id='LC295'><span class="k">if</span> <span class="n">__name__</span> <span class="o">==</span> <span class="s">&#39;__main__&#39;</span><span class="p">:</span></div><div class='line' id='LC296'>&nbsp;&nbsp;&nbsp;&nbsp;<span class="n">test</span><span class="p">()</span></div></pre></div>
          </td>
        </tr>
      </table>
  </div>

          </div>
        </div>

        <a href="#jump-to-line" rel="facebox" data-hotkey="l" class="js-jump-to-line" style="display:none">Jump to Line</a>
        <div id="jump-to-line" style="display:none">
          <h2>Jump to Line</h2>
          <form accept-charset="UTF-8" class="js-jump-to-line-form">
            <input class="textfield js-jump-to-line-field" type="text">
            <div class="full-button">
              <button type="submit" class="button">Go</button>
            </div>
          </form>
        </div>

      </div>
    </div>
</div>

<div id="js-frame-loading-template" class="frame frame-loading large-loading-area" style="display:none;">
  <img class="js-frame-loading-spinner" src="https://a248.e.akamai.net/assets.github.com/images/spinners/octocat-spinner-128.gif?1360648843" height="64" width="64">
</div>


        </div>
      </div>
      <div class="context-overlay"></div>
    </div>

      <div id="footer-push"></div><!-- hack for sticky footer -->
    </div><!-- end of wrapper - hack for sticky footer -->

      <!-- footer -->
      <div id="footer">
  <div class="container clearfix">

      <dl class="footer_nav">
        <dt>GitHub</dt>
        <dd><a href="https://github.com/about">About us</a></dd>
        <dd><a href="https://github.com/blog">Blog</a></dd>
        <dd><a href="https://github.com/contact">Contact &amp; support</a></dd>
        <dd><a href="http://enterprise.github.com/">GitHub Enterprise</a></dd>
        <dd><a href="http://status.github.com/">Site status</a></dd>
      </dl>

      <dl class="footer_nav">
        <dt>Applications</dt>
        <dd><a href="http://mac.github.com/">GitHub for Mac</a></dd>
        <dd><a href="http://windows.github.com/">GitHub for Windows</a></dd>
        <dd><a href="http://eclipse.github.com/">GitHub for Eclipse</a></dd>
        <dd><a href="http://mobile.github.com/">GitHub mobile apps</a></dd>
      </dl>

      <dl class="footer_nav">
        <dt>Services</dt>
        <dd><a href="http://get.gaug.es/">Gauges: Web analytics</a></dd>
        <dd><a href="http://speakerdeck.com">Speaker Deck: Presentations</a></dd>
        <dd><a href="https://gist.github.com">Gist: Code snippets</a></dd>
        <dd><a href="http://jobs.github.com/">Job board</a></dd>
      </dl>

      <dl class="footer_nav">
        <dt>Documentation</dt>
        <dd><a href="http://help.github.com/">GitHub Help</a></dd>
        <dd><a href="http://developer.github.com/">Developer API</a></dd>
        <dd><a href="http://github.github.com/github-flavored-markdown/">GitHub Flavored Markdown</a></dd>
        <dd><a href="http://pages.github.com/">GitHub Pages</a></dd>
      </dl>

      <dl class="footer_nav">
        <dt>More</dt>
        <dd><a href="http://training.github.com/">Training</a></dd>
        <dd><a href="https://github.com/edu">Students &amp; teachers</a></dd>
        <dd><a href="http://shop.github.com">The Shop</a></dd>
        <dd><a href="/plans">Plans &amp; pricing</a></dd>
        <dd><a href="http://octodex.github.com/">The Octodex</a></dd>
      </dl>

      <hr class="footer-divider">


    <p class="right">&copy; 2013 <span title="0.07554s from fe3.rs.github.com">GitHub</span>, Inc. All rights reserved.</p>
    <a class="left" href="https://github.com/">
      <span class="mega-icon mega-icon-invertocat"></span>
    </a>
    <ul id="legal">
        <li><a href="https://github.com/site/terms">Terms of Service</a></li>
        <li><a href="https://github.com/site/privacy">Privacy</a></li>
        <li><a href="https://github.com/security">Security</a></li>
    </ul>

  </div><!-- /.container -->

</div><!-- /.#footer -->


    <div class="fullscreen-overlay js-fullscreen-overlay" id="fullscreen_overlay">
  <div class="fullscreen-container js-fullscreen-container">
    <div class="textarea-wrap">
      <textarea name="fullscreen-contents" id="fullscreen-contents" class="js-fullscreen-contents" placeholder="" data-suggester="fullscreen_suggester"></textarea>
          <div class="suggester-container">
              <div class="suggester fullscreen-suggester js-navigation-container" id="fullscreen_suggester"
                 data-url="/tualatrix/tools/suggestions/commit">
              </div>
          </div>
    </div>
  </div>
  <div class="fullscreen-sidebar">
    <a href="#" class="exit-fullscreen js-exit-fullscreen tooltipped leftwards" title="Exit Zen Mode">
      <span class="mega-icon mega-icon-normalscreen"></span>
    </a>
    <a href="#" class="theme-switcher js-theme-switcher tooltipped leftwards"
      title="Switch themes">
      <span class="mini-icon mini-icon-brightness"></span>
    </a>
  </div>
</div>



    <div id="ajax-error-message" class="flash flash-error">
      <span class="mini-icon mini-icon-exclamation"></span>
      Something went wrong with that request. Please try again.
      <a href="#" class="mini-icon mini-icon-remove-close ajax-error-dismiss"></a>
    </div>

    
    
    <span id='server_response_time' data-time='0.07622' data-host='fe3'></span>
    
  </body>
</html>

